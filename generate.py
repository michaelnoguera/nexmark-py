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
        if event_type == "all" or event.kind() == event_type:
            yield event.value
        idx += step

def format_event(event, fmt: str) -> str:
    if fmt == "json":
        return json.dumps(event.to_dict(), separators=(',', ':'))
    elif fmt == "rust":
        cls = event.__class__.__name__
        fields = ", ".join(f'{k}: {json.dumps(v)}' for k, v in event.to_dict().items())
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
        delay = 0.00005 if not args.no_wait else 0
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