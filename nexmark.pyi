class Config:
    num_event_generators: int
    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...

class Person:
    def __init__(
        self,
        id: int,
        name: str,
        email_address: str,
        credit_card: str,
        city: str,
        state: str,
        date_time: int,
        extra: str,
    ) -> None: ...
    id: int
    name: str
    email_address: str
    credit_card: str
    city: str
    state: str
    date_time: int
    extra: str
    def __repr__(self) -> str: ...
    def to_dict(self) -> dict: ...
    def to_json(self) -> str: ...

class Auction:
    def __init__(
        self,
        id: int,
        item_name: str,
        description: str,
        initial_bid: int,
        reserve: int,
        date_time: int,
        expires: int,
        seller: int,
        category: int,
        extra: str,
    ) -> None: ...
    id: int
    item_name: str
    description: str
    initial_bid: int
    reserve: int
    date_time: int
    expires: int
    seller: int
    category: int
    extra: str
    def __repr__(self) -> str: ...
    def to_dict(self) -> dict: ...
    def to_json(self) -> str: ...

class Bid:
    def __init__(
        self,
        auction: int,
        bidder: int,
        price: int,
        date_time: int,
        channel: str,
        url: str,
        extra: str,
    ) -> None: ...
    auction: int
    bidder: int
    price: int
    date_time: int
    channel: str
    url: str
    extra: str
    def __repr__(self) -> str: ...
    def to_dict(self) -> dict: ...
    def to_json(self) -> str: ...

class Event:
    @property
    def value(self) -> Person | Auction | Bid: ...
    def kind(self) -> str: ...
    def get_person(self) -> Person | None: ...
    def get_auction(self) -> Auction | None: ...
    def get_bid(self) -> Bid | None: ...
    def is_person(self) -> bool: ...
    def is_auction(self) -> bool: ...
    def is_bid(self) -> bool: ...
    def to_json(self) -> str: ...
    def to_dict(self) -> dict: ...
    def __repr__(self) -> str: ...

class EventGenerator:
    def __init__(self, config: Config = ...) -> None: ...
    def __iter__(self) -> EventGenerator: ...
    def __next__(self) -> Event | None: ...
    def take(self, n: int) -> list[Event]: ...
