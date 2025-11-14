use pyo3::{prelude::*, types::PyDict};

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

#[pyclass(name = "Person", dict)]
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
    #[new]
    fn new(
        id: usize,
        name: String,
        email_address: String,
        credit_card: String,
        city: String,
        state: String,
        date_time: u64,
        extra: String,
    ) -> Self {
        PyPerson {
            id,
            name,
            email_address,
            credit_card,
            city,
            state,
            date_time,
            extra,
        }
    }
    fn __repr__(&self) -> String {
        format!(
            "Person(id={}, name='{}', email='{}', city='{}', state='{}')",
            self.id, self.name, self.email_address, self.city, self.state
        )
    }
    fn to_dict(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            let dict = PyDict::new(py);
            dict.set_item("id", self.id)?;
            dict.set_item("name", self.name.clone())?;
            dict.set_item("email_address", self.email_address.clone())?;
            dict.set_item("credit_card", self.credit_card.clone())?;
            dict.set_item("city", self.city.clone())?;
            dict.set_item("state", self.state.clone())?;
            dict.set_item("date_time", self.date_time)?;
            dict.set_item("extra", self.extra.clone())?;
            Ok(dict.into())
        })
    }
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize Person to JSON: {}",
                e
            ))
        })
    }
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        serde_json::from_str(json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to deserialize Person from JSON: {}", e
            ))
        })
    }
}


#[pyclass(name = "Auction", dict)]
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
    #[new]
    fn new(
        id: usize,
        item_name: String,
        description: String,
        initial_bid: usize,
        reserve: usize,
        date_time: u64,
        expires: u64,
        seller: usize,
        category: usize,
        extra: String,
    ) -> Self {
        PyAuction {
            id,
            item_name,
            description,
            initial_bid,
            reserve,
            date_time,
            expires,
            seller,
            category,
            extra,
        }
    }
    fn __repr__(&self) -> String {
        format!(
            "Auction(id={}, item='{}', initial_bid={}, seller={})",
            self.id, self.item_name, self.initial_bid, self.seller
        )
    }
    fn to_dict(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            let dict = PyDict::new(py);
            dict.set_item("id", self.id)?;
            dict.set_item("item_name", self.item_name.clone())?;
            dict.set_item("description", self.description.clone())?;
            dict.set_item("initial_bid", self.initial_bid)?;
            dict.set_item("reserve", self.reserve)?;
            dict.set_item("date_time", self.date_time)?;
            dict.set_item("expires", self.expires)?;
            dict.set_item("seller", self.seller)?;
            dict.set_item("category", self.category)?;
            dict.set_item("extra", self.extra.clone())?;
            Ok(dict.into())
        })
    }
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize Auction to JSON: {}",
                e
            ))
        })
    }
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        serde_json::from_str(json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to deserialize Auction from JSON: {}", e
            ))
        })
    }
}


#[pyclass(name = "Bid", dict)]
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
    #[new]
    fn new(
        auction: i64,
        bidder: i64,
        price: i64,
        date_time: i64,
        channel: String,
        url: String,
        extra: String,
    ) -> Self {
        PyBid {
            auction: auction.try_into().unwrap(),
            bidder: bidder.try_into().unwrap(),
            price: price.try_into().unwrap(),
            date_time: date_time.try_into().unwrap(),
            channel,
            url,
            extra,
        }
    }
    fn __repr__(&self) -> String {
        format!(
            "Bid(auction={}, bidder={}, price={})",
            self.auction, self.bidder, self.price
        )
    }
    fn to_dict(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            let dict = PyDict::new(py);
            dict.set_item("auction", self.auction)?;
            dict.set_item("bidder", self.bidder)?;
            dict.set_item("price", self.price)?;
            dict.set_item("date_time", self.date_time)?;
            dict.set_item("channel", self.channel.clone())?;
            dict.set_item("url", self.url.clone())?;
            dict.set_item("extra", self.extra.clone())?;
            Ok(dict.into())
        })
    }
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to serialize Bid to JSON: {}",
                e
            ))
        })
    }
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        serde_json::from_str(json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to deserialize Bid from JSON: {}", e
            ))
        })
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
    #[new]
    fn new(obj: &Bound<PyAny>) -> PyResult<Self> {
        if let Ok(person) = obj.extract::<PyPerson>() {
            return Ok(PyEvent { inner: PyEventEnum::Person(person) });
        }
        if let Ok(auction) = obj.extract::<PyAuction>() {
            return Ok(PyEvent { inner: PyEventEnum::Auction(auction) });
        }
        if let Ok(bid) = obj.extract::<PyBid>() {
            return Ok(PyEvent { inner: PyEventEnum::Bid(bid) });
        }
        if let Ok(json_str) = obj.extract::<String>() {
            if let Ok(person) = serde_json::from_str::<PyPerson>(&json_str) {
                return Ok(PyEvent { inner: PyEventEnum::Person(person) });
            }
            if let Ok(auction) = serde_json::from_str::<PyAuction>(&json_str) {
                return Ok(PyEvent { inner: PyEventEnum::Auction(auction) });
            }
            if let Ok(bid) = serde_json::from_str::<PyBid>(&json_str) {
                return Ok(PyEvent { inner: PyEventEnum::Bid(bid) });
            }
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "JSON did not match Person, Auction, or Bid schema",
            ));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Argument must be a Person, Auction, Bid, or JSON string",
        ))
    }
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
    fn get_person(&self) -> Option<PyPerson> {
        match &self.inner {
            PyEventEnum::Person(p) => Some(p.clone()),
            _ => None,
        }
    }
    fn get_auction(&self) -> Option<PyAuction> {
        match &self.inner {
            PyEventEnum::Auction(a) => Some(a.clone()),
            _ => None,
        }
    }
    fn get_bid(&self) -> Option<PyBid> {
        match &self.inner {
            PyEventEnum::Bid(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn is_person(&self) -> bool {
        matches!(self.inner, PyEventEnum::Person(_))
    }
    fn is_auction(&self) -> bool {
        matches!(self.inner, PyEventEnum::Auction(_))
    }
    fn is_bid(&self) -> bool {
        matches!(self.inner, PyEventEnum::Bid(_))
    }
    fn to_json(&self) -> PyResult<String> {
        match &self.inner {
            PyEventEnum::Person(p) => p.to_json(),
            PyEventEnum::Auction(a) => a.to_json(),
            PyEventEnum::Bid(b) => b.to_json(),
        }
    }
    fn to_dict(&self) -> PyResult<Py<PyAny>> {
        match &self.inner {
            PyEventEnum::Person(p) => p.to_dict(),
            PyEventEnum::Auction(a) => a.to_dict(),
            PyEventEnum::Bid(b) => b.to_dict(),
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

