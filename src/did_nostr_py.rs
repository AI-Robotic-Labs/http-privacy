//! Python PyO3 bindings for DID-NOSTR functionality

use pyo3::prelude::*;
use crate::did_nostr::*;

/// Python wrapper for NostrPublicKey
#[pyclass]
pub struct PyNostrPublicKey {
    inner: NostrPublicKey,
}

#[pymethods]
impl PyNostrPublicKey {
    #[new]
    fn new(hex: String) -> PyResult<Self> {
        let inner = NostrPublicKey::from_hex(&hex)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        Ok(PyNostrPublicKey { inner })
    }

    fn as_hex(&self) -> String {
        self.inner.as_hex().to_string()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("PyNostrPublicKey('{}')", self.inner.as_hex())
    }
}

/// Python wrapper for DidNostr
#[pyclass]
pub struct PyDidNostr {
    inner: DidNostr,
}

#[pymethods]
impl PyDidNostr {
    /// Create DID from public key
    #[staticmethod]
    fn from_pubkey(pubkey: &PyNostrPublicKey) -> Self {
        PyDidNostr {
            inner: DidNostr::from_pubkey(pubkey.inner.clone()),
        }
    }

    /// Parse DID from string
    #[staticmethod]
    fn from_str(did_str: String) -> PyResult<Self> {
        let inner = DidNostr::from_str(&did_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        Ok(PyDidNostr { inner })
    }

    /// Get public key
    fn pubkey(&self) -> PyNostrPublicKey {
        PyNostrPublicKey {
            inner: self.inner.pubkey().clone(),
        }
    }

    /// Get DID as string
    fn to_string(&self) -> String {
        self.inner.to_string()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("PyDidNostr('{}')", self.inner.to_string())
    }
}

/// Python wrapper for NostrSignature
#[pyclass]
pub struct PyNostrSignature {
    inner: NostrSignature,
}

#[pymethods]
impl PyNostrSignature {
    #[new]
    fn new(hex: String) -> PyResult<Self> {
        let inner = NostrSignature::from_hex(&hex)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        Ok(PyNostrSignature { inner })
    }

    fn as_hex(&self) -> String {
        self.inner.as_hex().to_string()
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

/// Python wrapper for VerificationResult
#[pyclass]
pub struct PyVerificationResult {
    valid: bool,
    did: Option<PyDidNostr>,
    error: Option<String>,
}

#[pymethods]
impl PyVerificationResult {
    #[getter]
    fn valid(&self) -> bool {
        self.valid
    }

    #[getter]
    fn did(&self) -> Option<PyDidNostr> {
        self.did.clone()
    }

    #[getter]
    fn error(&self) -> Option<String> {
        self.error.clone()
    }

    fn __repr__(&self) -> String {
        if self.valid {
            format!("VerificationResult(valid=True, did='{}')", self.did.as_ref().unwrap().inner.to_string())
        } else {
            format!("VerificationResult(valid=False, error='{}')", self.error.as_ref().unwrap_or(&"Unknown".to_string()))
        }
    }
}

// Helper to convert internal VerificationResult to Python type
impl From<VerificationResult> for PyVerificationResult {
    fn from(result: VerificationResult) -> Self {
        PyVerificationResult {
            valid: result.valid,
            did: result.did.map(|d| PyDidNostr { inner: d }),
            error: result.error,
        }
    }
}

impl Clone for PyDidNostr {
    fn clone(&self) -> Self {
        PyDidNostr {
            inner: self.inner.clone(),
        }
    }
}

impl Clone for PyVerificationResult {
    fn clone(&self) -> Self {
        PyVerificationResult {
            valid: self.valid,
            did: self.did.clone(),
            error: self.error.clone(),
        }
    }
}

/// Python wrapper for NostrVerifier
#[pyclass]
pub struct PyNostrVerifier;

#[pymethods]
impl PyNostrVerifier {
    /// Verify a NOSTR signature
    #[staticmethod]
    fn verify(
        pubkey: &PyNostrPublicKey,
        message: String,
        signature: &PyNostrSignature,
    ) -> PyVerificationResult {
        let result = NostrVerifier::verify(&pubkey.inner, &message, &signature.inner);
        result.into()
    }
}

/// Python wrapper for RequestCanonicalizer
#[pyclass]
pub struct PyRequestCanonicalizer;

#[pymethods]
impl PyRequestCanonicalizer {
    /// Create canonical form of HTTP request for signing
    #[staticmethod]
    fn canonicalize(
        method: String,
        path: String,
        headers: Vec<(String, String)>,
        body: String,
    ) -> String {
        RequestCanonicalizer::canonicalize(&method, &path, &headers, &body)
    }
}

/// Python module initialization
pub fn init_did_nostr_module(py: Python<'_>, parent_module: &pyo3::types::PyModule) -> PyResult<()> {
    let submodule = pyo3::types::PyModule::new(py, "did_nostr")?;
    submodule.add_class::<PyNostrPublicKey>()?;
    submodule.add_class::<PyDidNostr>()?;
    submodule.add_class::<PyNostrSignature>()?;
    submodule.add_class::<PyVerificationResult>()?;
    submodule.add_class::<PyNostrVerifier>()?;
    submodule.add_class::<PyRequestCanonicalizer>()?;
    parent_module.add_submodule(submodule)?;
    Ok(())
}
