use std::collections::HashMap;

use crate::vector::Vector;

struct Map<TTile> {
    tiles: HashMap<Vector, TTile>,
}
