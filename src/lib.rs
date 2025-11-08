//! Python bindings for nexmark-rs using PyO3.

use pyo3::prelude::*;

use ::nexmark::config::NexmarkConfig;
use ::nexmark::event::Event;
use ::nexmark::EventGenerator;

use serde::{Deserialize, Serialize};

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
#[derive(Clone, Serialize, Deserialize, Debug)]
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


#[pyclass(name = "Auction")]
#[derive(Clone, Serialize, Deserialize, Debug)]
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


#[pyclass(name = "Bid")]
#[derive(Clone, Serialize, Deserialize, Debug)]
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


#[pyclass(name = "Event")]
#[derive(Clone, Debug)]
pub enum PyEventEnum {
    Person(PyPerson),
    Auction(PyAuction),
    Bid(PyBid),
}

#[pyclass(name = "Event")]
pub struct PyEvent {
    pub inner: PyEventEnum,
}

#[pymethods]
impl PyEvent {
    fn __repr__(&self) -> String {
        match &self.inner {
            PyEventEnum::Person(p) => p.__repr__(),
            PyEventEnum::Auction(a) => a.__repr__(),
            PyEventEnum::Bid(b) => b.__repr__(),
        }
    }
    #[getter]
    fn value(&self) -> Py<PyAny> {
        Python::attach(|py| match &self.inner {
            PyEventEnum::Person(p) => Py::new(py, p.clone()).unwrap().into(),
            PyEventEnum::Auction(a) => Py::new(py, a.clone()).unwrap().into(),
            PyEventEnum::Bid(b) => Py::new(py, b.clone()).unwrap().into(),
        })
    }
    fn kind(&self) -> &'static str {
        match &self.inner {
            PyEventEnum::Person(_) => "person",
            PyEventEnum::Auction(_) => "auction",
            PyEventEnum::Bid(_) => "bid",
        }
    }
}

impl From<Event> for PyEvent {
    fn from(event: Event) -> Self {
        let inner = match event {
            Event::Person(person) => PyEventEnum::Person(PyPerson {
                id: person.id,
                name: person.name,
                email_address: person.email_address,
                credit_card: person.credit_card,
                city: person.city,
                state: person.state,
                date_time: person.date_time,
                extra: person.extra,
            }),
            Event::Auction(auction) => PyEventEnum::Auction(PyAuction {
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
            }),
            Event::Bid(bid) => PyEventEnum::Bid(PyBid {
                auction: bid.auction,
                bidder: bid.bidder,
                price: bid.price,
                date_time: bid.date_time,
                channel: bid.channel,
                url: bid.url,
                extra: bid.extra,
            }),
        };
        PyEvent { inner }
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
            slf.inner
                .next()
                .and_then(|event| Py::new(py, PyEvent::from(event)).ok())
                .map(|obj| obj.into())
        })
    }

    fn take(&mut self, n: usize) -> Vec<Py<PyAny>> {
        Python::attach(|py| {
            self.inner
                .by_ref()
                .take(n)
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
