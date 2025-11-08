import pytest
import nexmark

def test_generate_ten_events():
    config = nexmark.Config()
    generator = nexmark.EventGenerator(config)
    events = []
    for i, event in enumerate(generator):
        if i >= 10:
            break
        events.append(event)
    assert len(events) == 10
    
def test_config_defaults():
    cfg = nexmark.Config()
    assert isinstance(cfg, nexmark.Config)
    assert cfg.num_event_generators == 1

def test_config_setters():
    cfg = nexmark.Config()
    cfg.num_event_generators = 5
    assert cfg.num_event_generators == 5

def test_event_generator_iter_take():
    cfg = nexmark.Config()
    gen = nexmark.EventGenerator(cfg)
    events = list(gen.take(10))
    assert len(events) == 10
    for event in events:
        assert isinstance(event, nexmark.Event)

def test_event_generator_next():
    cfg = nexmark.Config()
    gen = nexmark.EventGenerator(cfg)
    it = iter(gen)
    event = next(it)
    assert isinstance(event, nexmark.Event)

def test_event_types_and_methods():
    cfg = nexmark.Config()
    gen = nexmark.EventGenerator(cfg)
    event = next(iter(gen))
    assert isinstance(event, nexmark.Event)
    assert event.is_person() or event.is_auction() or event.is_bid()
    if event.is_person():
        person = event.get_person()
        assert isinstance(person, nexmark.Person)
        assert isinstance(person.id, int)
        assert isinstance(person.name, str)
    elif event.is_auction():
        auction = event.get_auction()
        assert isinstance(auction, nexmark.Auction)
        assert isinstance(auction.id, int)
        assert isinstance(auction.item_name, str)
    elif event.is_bid():
        bid = event.get_bid()
        assert isinstance(bid, nexmark.Bid)
        assert isinstance(bid.auction, int)
        assert isinstance(bid.bidder, int)

def test_person_repr():
    cfg = nexmark.Config()
    gen = nexmark.EventGenerator(cfg)
    for event in gen.take(10):
        if event.is_person():
            person = event.get_person()
            assert "Person(" in repr(person)

def test_auction_repr():
    cfg = nexmark.Config()
    gen = nexmark.EventGenerator(cfg)
    for event in gen.take(10):
        if event.is_auction():
            auction = event.get_auction()
            assert "Auction(" in repr(auction)

def test_bid_repr():
    cfg = nexmark.Config()
    gen = nexmark.EventGenerator(cfg)
    for event in gen.take(10):
        if event.is_bid():
            bid = event.get_bid()
            assert "Bid(" in repr(bid)