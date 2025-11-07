//! Python bindings for nexmark-rs using PyO3.

use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;
use ::nexmark::config::NexmarkConfig;
use ::nexmark::event::{Event, Person, Auction, Bid};
use ::nexmark::EventGenerator;

#[pyclass(name = "Config")]
#[derive(Clone)]
pub struct PyConfig {
    inner: NexmarkConfig,
}

#[pymethods]
impl PyConfig {
    #[new]
    fn new() -> Self {
        PyConfig {
            inner: NexmarkConfig::default(),
        }
    }

    #[getter]
    fn num_event_generators(&self) -> usize {
        self.inner.num_event_generators
    }

    #[setter]
    fn set_num_event_generators(&mut self, value: usize) {
        self.inner.num_event_generators = value;
    }
}

#[pyclass(name = "Person")]
#[derive(Clone)]
pub struct PyPerson {
    #[pyo3(get)]
    pub id: usize,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub email_address: String,
    #[pyo3(get)]
    pub credit_card: String,
    #[pyo3(get)]
    pub city: String,
    #[pyo3(get)]
    pub state: String,
    #[pyo3(get)]
    pub date_time: u64,
    #[pyo3(get)]
    pub extra: String,
}

#[pymethods]
impl PyPerson {
    fn __repr__(&self) -> String {
        format!(
            "Person(id={}, name='{}', email='{}', city='{}', state='{}')",
            self.id, self.name, self.email_address, self.city, self.state
        )
    }
}

impl From<Person> for PyPerson {
    fn from(person: Person) -> Self {
        PyPerson {
            id: person.id,
            name: person.name,
            email_address: person.email_address,
            credit_card: person.credit_card,
            city: person.city,
            state: person.state,
            date_time: person.date_time,
            extra: person.extra,
        }
    }
}

#[pyclass(name = "Auction")]
#[derive(Clone)]
pub struct PyAuction {
    #[pyo3(get)]
    pub id: usize,
    #[pyo3(get)]
    pub item_name: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub initial_bid: usize,
    #[pyo3(get)]
    pub reserve: usize,
    #[pyo3(get)]
    pub date_time: u64,
    #[pyo3(get)]
    pub expires: u64,
    #[pyo3(get)]
    pub seller: usize,
    #[pyo3(get)]
    pub category: usize,
    #[pyo3(get)]
    pub extra: String,
}

#[pymethods]
impl PyAuction {
    fn __repr__(&self) -> String {
        format!(
            "Auction(id={}, item='{}', initial_bid={}, seller={})",
            self.id, self.item_name, self.initial_bid, self.seller
        )
    }
}

impl From<Auction> for PyAuction {
    fn from(auction: Auction) -> Self {
        PyAuction {
            id: auction.id,
            item_name: auction.item_name,
            description: auction.description,
            initial_bid: auction.initial_bid,
            reserve: auction.reserve,
            date_time: auction.date_time,
            expires: auction.expires,
            seller: auction.seller,
            category: auction.category,
            extra: auction.extra,
        }
    }
}

#[pyclass(name = "Bid")]
#[derive(Clone)]
pub struct PyBid {
    #[pyo3(get)]
    pub auction: usize,
    #[pyo3(get)]
    pub bidder: usize,
    #[pyo3(get)]
    pub price: usize,
    #[pyo3(get)]
    pub date_time: u64,
    #[pyo3(get)]
    pub channel: String,
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub extra: String,
}

#[pymethods]
impl PyBid {
    fn __repr__(&self) -> String {
        format!(
            "Bid(auction={}, bidder={}, price={})",
            self.auction, self.bidder, self.price
        )
    }
}

impl From<Bid> for PyBid {
    fn from(bid: Bid) -> Self {
        PyBid {
            auction: bid.auction,
            bidder: bid.bidder,
            price: bid.price,
            date_time: bid.date_time,
            channel: bid.channel,
            url: bid.url,
            extra: bid.extra,
        }
    }
}

#[pyclass(name = "Event")]
#[derive(Clone)]
pub struct PyEvent {
    pub kind: String,
    pub person: Option<PyPerson>,
    pub auction: Option<PyAuction>,
    pub bid: Option<PyBid>,
}

#[pymethods]
impl PyEvent {
    fn __repr__(&self) -> String {
        match self.kind.as_str() {
            "Person" => self.person.as_ref().map_or("Person(None)".to_string(), |p| p.__repr__()),
            "Auction" => self.auction.as_ref().map_or("Auction(None)".to_string(), |a| a.__repr__()),
            "Bid" => self.bid.as_ref().map_or("Bid(None)".to_string(), |b| b.__repr__()),
            _ => "Unknown".to_string(),
        }
    }
    fn is_person(&self) -> bool { self.kind == "Person" }
    fn is_auction(&self) -> bool { self.kind == "Auction" }
    fn is_bid(&self) -> bool { self.kind == "Bid" }
    fn get_person(&self) -> PyResult<PyPerson> {
        self.person.clone().ok_or_else(|| PyTypeError::new_err("Event is not a Person"))
    }
    fn get_auction(&self) -> PyResult<PyAuction> {
        self.auction.clone().ok_or_else(|| PyTypeError::new_err("Event is not an Auction"))
    }
    fn get_bid(&self) -> PyResult<PyBid> {
        self.bid.clone().ok_or_else(|| PyTypeError::new_err("Event is not a Bid"))
    }
}

impl From<Event> for PyEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Person(person) => PyEvent {
                kind: "Person".to_string(),
                person: Some(person.into()),
                auction: None,
                bid: None,
            },
            Event::Auction(auction) => PyEvent {
                kind: "Auction".to_string(),
                person: None,
                auction: Some(auction.into()),
                bid: None,
            },
            Event::Bid(bid) => PyEvent {
                kind: "Bid".to_string(),
                person: None,
                auction: None,
                bid: Some(bid.into()),
            },
        }
    }
}



#[pyclass(name = "EventGenerator")]
pub struct PyEventGenerator {
    inner: EventGenerator,
}

#[pymethods]
impl PyEventGenerator {
    #[new]
    fn new(config: Option<PyConfig>) -> Self {
        let rust_config = config.map(|c| c.inner).unwrap_or_default();
        PyEventGenerator {
            inner: EventGenerator::new(rust_config),
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Py<PyAny>> {
        Python::attach(|py| {
            slf.inner.next()
                .and_then(|event| Py::new(py, PyEvent::from(event)).ok())
                .map(|obj| obj.into())
        })
    }

    fn take(&mut self, n: usize) -> Vec<Py<PyAny>> {
        Python::attach(|py| {
            self.inner.by_ref().take(n)
                .filter_map(|event| Py::new(py, PyEvent::from(event)).ok())
                .map(|obj| obj.into())
                .collect()
        })
    }
}

#[pymodule]
fn nexmark(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyConfig>()?;
    m.add_class::<PyPerson>()?;
    m.add_class::<PyAuction>()?;
    m.add_class::<PyBid>()?;
    m.add_class::<PyEvent>()?;
    m.add_class::<PyEventGenerator>()?;
    Ok(())
}
