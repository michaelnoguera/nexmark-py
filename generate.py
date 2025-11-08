import sys
import time
import json
import argparse
from typing import Iterator, Any
import itertools
import nexmark

EVENT_TYPES = ["person", "auction", "bid"]

def event_stream(event_type: str, offset: int, step: int) -> Iterator[Any]:
    config = nexmark.Config()
    gen = nexmark.EventGenerator(config)
    idx = 0
    for event in gen:
        if idx < offset:
            idx += step
            continue
        if event_type == "person" and event.is_person():
            yield event.get_person()
        elif event_type == "auction" and event.is_auction():
            yield event.get_auction()
        elif event_type == "bid" and event.is_bid():
            yield event.get_bid()
        elif event_type == "all":
            if event.is_person():
                yield event.get_person()
            elif event.is_auction():
                yield event.get_auction()
            elif event.is_bid():
                yield event.get_bid()
        idx += step

def format_event(event, fmt: str) -> str:
    def to_dict(obj):
        # Use attribute order from pyi for Person, Auction, Bid
        cls = obj.__class__.__name__
        if cls == "Person":
            keys = [
                "id", "name", "email_address", "credit_card",
                "city", "state", "date_time", "extra"
            ]
        elif cls == "Auction":
            keys = [
                "id", "item_name", "description", "initial_bid",
                "reserve", "date_time", "expires", "seller",
                "category", "extra"
            ]
        elif cls == "Bid":
            keys = [
                "auction", "bidder", "price", "date_time",
                "channel", "url", "extra"
            ]
        else:
            keys = [a for a in dir(obj) if not a.startswith("_") and not callable(getattr(obj, a))]
        return {k: getattr(obj, k) for k in keys if hasattr(obj, k)}
    if fmt == "json":
        cls = event.__class__.__name__
        if cls in ("Person", "Auction", "Bid"):
            return json.dumps({cls: to_dict(event)}, separators=(',', ':'))
        return json.dumps(to_dict(event), separators=(',', ':'))
    elif fmt == "rust":
        cls = event.__class__.__name__
        fields = ", ".join(f'{k}: {json.dumps(v)}' for k, v in to_dict(event).items())
        return f'{cls} {{ {fields} }}'
    return str(event)

def main(argv=None):
    if argv is None:
        argv = sys.argv[1:]
    parser = argparse.ArgumentParser(
        prog="generate.py",
        description="Nexmark event generator"
    )
    parser.add_argument("-t", "--type", choices=["all"] + EVENT_TYPES, default="all", help="The type of events to generate [default: all]")
    parser.add_argument("-n", "--number", type=int, help="The number of events to generate. If not specified, generate events forever")
    parser.add_argument("--offset", type=int, default=0, help="The start event offset [default: 0]")
    parser.add_argument("--step", type=int, default=1, help="The step for each iteration [default: 1]")
    parser.add_argument("--format", choices=["json", "rust"], default="json", help="Print format [default: json]")
    parser.add_argument("--no-wait", action="store_true", help="Generate all events immediately")
    args = parser.parse_args(argv)

    stream = event_stream(args.type, args.offset, args.step)
    
    try:
        # Remove artificial delay unless explicitly requested
        delay = 1 if not args.no_wait else 0
        if args.format == "json":
            if args.number is not None:
                # Efficient batch output
                for event in itertools.islice(stream, args.number):
                    print(format_event(event, "json"))
            else:
                # Stream output, no delay unless requested
                for event in stream:
                    print(format_event(event, "json"))
                    if delay:
                        time.sleep(delay)
        else:
            if args.number is not None:
                for _, event in zip(range(args.number), stream):
                    print(format_event(event, args.format))
                    if delay:
                        time.sleep(delay)
            else:
                for event in stream:
                    print(format_event(event, args.format))
                    if delay:
                        time.sleep(delay)
    except KeyboardInterrupt:
        sys.exit(0)

if __name__ == "__main__":
    main()