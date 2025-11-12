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
def test_event_to_dict():
    cfg = nexmark.Config()
    gen = nexmark.EventGenerator(cfg)
    for event in gen.take(10):
        d = event.to_dict()
        assert isinstance(d, dict)
        if event.is_person():
            assert "id" in d and "name" in d
        elif event.is_auction():
            assert "id" in d and "item_name" in d
        elif event.is_bid():
            assert "auction" in d and "bidder" in d
def test_person_to_dict():
    from nexmark import Person
    person = Person(
        id=1,
        name="Alice",
        email_address="alice@example.com",
        credit_card="1234",
        city="Wonderland",
        state="TX",
        date_time=1234567890,
        extra="extra"
    )
    d = person.to_dict()
    assert d["id"] == 1
    assert d["name"] == "Alice"
    assert d["email_address"] == "alice@example.com"
    assert d["credit_card"] == "1234"
    assert d["city"] == "Wonderland"
    assert d["state"] == "TX"
    assert d["date_time"] == 1234567890
    assert d["extra"] == "extra"

def test_auction_to_dict():
    from nexmark import Auction
    auction = Auction(
        id=2,
        item_name="Book",
        description="A good book",
        initial_bid=10,
        reserve=20,
        date_time=1234567891,
        expires=1234567999,
        seller=3,
        category=4,
        extra="auction-extra"
    )
    d = auction.to_dict()
    assert d["id"] == 2
    assert d["item_name"] == "Book"
    assert d["description"] == "A good book"
    assert d["initial_bid"] == 10
    assert d["reserve"] == 20
    assert d["date_time"] == 1234567891
    assert d["expires"] == 1234567999
    assert d["seller"] == 3
    assert d["category"] == 4
    assert d["extra"] == "auction-extra"

def test_bid_to_dict():
    from nexmark import Bid
    bid = Bid(
        auction=5,
        bidder=6,
        price=100,
        date_time=1234567892,
        channel="web",
        url="http://example.com",
        extra="bid-extra"
    )
    d = bid.to_dict()
    assert d["auction"] == 5
    assert d["bidder"] == 6
    assert d["price"] == 100
    assert d["date_time"] == 1234567892
    assert d["channel"] == "web"
    assert d["url"] == "http://example.com"
    assert d["extra"] == "bid-extra"
def test_person_to_json():
    from nexmark import Person
    person = Person(
        id=1,
        name="Alice",
        email_address="alice@example.com",
        credit_card="1234",
        city="Wonderland",
        state="TX",
        date_time=1234567890,
        extra="extra"
    )
    json_str = person.to_json()
    assert isinstance(json_str, str)
    assert '"name":"Alice"' in json_str

def test_auction_to_json():
    from nexmark import Auction
    auction = Auction(
        id=2,
        item_name="Book",
        description="A good book",
        initial_bid=10,
        reserve=20,
        date_time=1234567891,
        expires=1234567999,
        seller=3,
        category=4,
        extra="auction-extra"
    )
    json_str = auction.to_json()
    assert isinstance(json_str, str)
    assert '"item_name":"Book"' in json_str

def test_bid_to_json():
    from nexmark import Bid
    bid = Bid(
        auction=5,
        bidder=6,
        price=100,
        date_time=1234567892,
        channel="web",
        url="http://example.com",
        extra="bid-extra"
    )
    json_str = bid.to_json()
    assert isinstance(json_str, str)
    assert '"auction":5' in json_str