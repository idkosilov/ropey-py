use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use ropey::Rope as RopeyRope;

#[pyclass]
struct Rope {
    rope: RopeyRope,
}

#[pymethods]
impl Rope {
    #[new]
    fn new(text: &str) -> Self {
        Self {
            rope: RopeyRope::from_str(text),
        }
    }

    fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    fn char(&self, idx: usize) -> PyResult<char> {
        if idx >= self.rope.len_chars() {
            return Err(PyIndexError::new_err("char index out of range"));
        }
        Ok(self.rope.char(idx))
    }

    fn line(&self, line_idx: usize) -> PyResult<String> {
        if line_idx >= self.rope.len_lines() {
            return Err(PyIndexError::new_err("line index out of range"));
        }
        Ok(self.rope.line(line_idx).to_string())
    }

    fn to_string(&self) -> String {
        self.rope.to_string()
    }

    fn insert(&mut self, idx: usize, text: &str) -> PyResult<()> {
        if idx > self.rope.len_chars() {
            return Err(PyIndexError::new_err("insert index out of range"));
        }
        self.rope.insert(idx, text);
        Ok(())
    }

    fn remove(&mut self, start: usize, end: usize) -> PyResult<()> {
        if start > end || end > self.rope.len_chars() {
            return Err(PyIndexError::new_err("remove range out of range"));
        }
        self.rope.remove(start..end);
        Ok(())
    }

    fn split_off(&mut self, at_char: usize) -> PyResult<Rope> {
        if at_char > self.rope.len_chars() {
            return Err(PyIndexError::new_err("split index out of range"));
        }
        let other = self.rope.split_off(at_char);
        Ok(Rope { rope: other })
    }

    fn slice(&self, start: usize, end: usize) -> PyResult<String> {
        if start > end || end > self.rope.len_chars() {
            return Err(PyIndexError::new_err("slice range out of range"));
        }
        Ok(self.rope.slice(start..end).to_string())
    }

    fn byte_slice(&self, start_byte: usize, end_byte: usize) -> PyResult<String> {
        if start_byte > end_byte || end_byte > self.rope.len_bytes() {
            return Err(PyIndexError::new_err("byte slice range out of range"));
        }
        Ok(self.rope.byte_slice(start_byte..end_byte).to_string())
    }

    fn byte_to_char(&self, byte_idx: usize) -> PyResult<usize> {
        if byte_idx > self.rope.len_bytes() {
            return Err(PyIndexError::new_err("byte index out of range"));
        }
        Ok(self.rope.byte_to_char(byte_idx))
    }

    fn char_to_byte(&self, char_idx: usize) -> PyResult<usize> {
        if char_idx > self.rope.len_chars() {
            return Err(PyIndexError::new_err("char index out of range"));
        }
        Ok(self.rope.char_to_byte(char_idx))
    }

    fn char_to_line(&self, char_idx: usize) -> PyResult<usize> {
        if char_idx > self.rope.len_chars() {
            return Err(PyIndexError::new_err("char index out of range"));
        }
        Ok(self.rope.char_to_line(char_idx))
    }

    fn line_to_char(&self, line_idx: usize) -> PyResult<usize> {
        if line_idx >= self.rope.len_lines() {
            return Err(PyIndexError::new_err("line index out of range"));
        }
        Ok(self.rope.line_to_char(line_idx))
    }

    fn line_to_byte(&self, line_idx: usize) -> PyResult<usize> {
        if line_idx >= self.rope.len_lines() {
            return Err(PyIndexError::new_err("line index out of range"));
        }
        Ok(self.rope.line_to_byte(line_idx))
    }

    fn byte_to_line(&self, byte_idx: usize) -> PyResult<usize> {
        if byte_idx > self.rope.len_bytes() {
            return Err(PyIndexError::new_err("byte index out of range"));
        }
        Ok(self.rope.byte_to_line(byte_idx))
    }
}

#[pymodule]
fn ropey_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rope>()?;
    Ok(())
}
