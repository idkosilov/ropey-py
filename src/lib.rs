use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ropey::Rope as RopeyRope;

#[pyclass]
struct Rope {
    rope: RopeyRope,
}

#[pymethods]
impl Rope {
    #[new]
    fn new(text: &str) -> Self {
        Self { rope: RopeyRope::from_str(text) }
    }

    fn insert(&mut self, char_idx: usize, text: &str) -> PyResult<()> {
        if char_idx > self.rope.len_chars() {
            return Err(PyIndexError::new_err("insert index out of range"));
        }
        self.rope.insert(char_idx, text);
        Ok(())
    }

    fn remove(&mut self, start_char: usize, end_char: usize) -> PyResult<()> {
        if start_char > end_char || end_char > self.rope.len_chars() || start_char > self.rope.len_chars() {
            return Err(PyIndexError::new_err("remove range out of range"));
        }
        self.rope.remove(start_char..end_char);
        Ok(())
    }

    fn get_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let full_text = self.rope.to_string();
        PyBytes::new(py, full_text.as_bytes())
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

    fn slice(&self, start: usize, end: usize) -> PyResult<String> {
        if start > end || end > self.rope.len_chars() {
            return Err(PyIndexError::new_err("slice range out of range"));
        }
        Ok(self.rope.slice(start..end).to_string())
    }

    fn byte_slice<'py>(&self, start_byte: usize, end_byte: usize, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        if start_byte > end_byte || end_byte > self.rope.len_bytes() {
            return Err(PyIndexError::new_err("byte slice range out of range"));
        }

        let slice = self.rope.byte_slice(start_byte..end_byte);
        let bytes: Vec<u8> = slice.bytes().collect();
        Ok(PyBytes::new(py, &bytes))
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
        if line_idx > self.rope.len_lines() {
            return Err(PyIndexError::new_err("line index out of range"));
        }
        Ok(self.rope.line_to_char(line_idx))
    }

    fn line_to_byte(&self, line_idx: usize) -> PyResult<usize> {
        if line_idx > self.rope.len_lines() {
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

    fn byte_to_point(&self, byte_idx: usize) -> PyResult<(usize, usize)> {
        if byte_idx > self.rope.len_bytes() {
            return Err(PyIndexError::new_err("byte index out of range"));
        }

        let line = self.rope.byte_to_line(byte_idx);

        let line_start_byte = if line < self.rope.len_lines() {
            self.rope.line_to_byte(line)
        } else {
            self.rope.len_bytes() - self.rope.line(line.saturating_sub(1)).len_bytes()
        };

        let column = byte_idx - line_start_byte;
        Ok((line, column))
    }


    fn point_to_byte(&self, line: usize, column: usize) -> PyResult<usize> {
        if line >= self.rope.len_lines() {
            return Err(PyIndexError::new_err("line index out of range"));
        }

        let line_start = self.rope.line_to_byte(line);
        let line_content = self.rope.line(line);
        let line_byte_len = line_content.len_bytes();

        if column > line_byte_len {
            return Err(PyIndexError::new_err("column index out of range"));
        }

        Ok(line_start + column)
    }
}

#[pymodule]
fn ropey_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rope>()?;
    Ok(())
}
