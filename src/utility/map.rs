//! Геометрические типы для 2D решетки в форме паззла.

//use std::ops::{Add, Mul, Sub, Neg, Index, IndexMut, Range};
use std::ops::{Index, IndexMut};

/// Двумерная точка решетки.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point(pub i32, pub i32);

/// Размер клетки.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Size(pub i32, pub i32);

/// Идентификатор, идентифицирующий ячейку в решетке прямоугольник.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CellId(usize);

/// An ID being given to cells on outside the rectangle.
// Идентификатор клеток за пределами прямоугольника.
pub const CELL_ID_OUTSIDE: CellId = CellId(0);

impl CellId {
    /// Создает новый объект `CellId` по ID.
    #[inline]
    pub fn new(id: usize) -> CellId {
        CellId(id)
    }

    /// Возвращает идентификатор ячейки.
    #[inline]
    pub fn id(self) -> usize {
        self.0
    }
}

/// Прямоугольная область.
pub trait Geom {
    /// Возвращает размер прямоугольника.
    #[inline]
    fn size(&self) -> Size;

    /// Возвращает количество строк прямоугольника.
    #[inline]
    fn row(&self) -> i32 {
        self.size().0
    }

    /// Возвращает количество столбов прямоугольника..
    #[inline]
    fn column(&self) -> i32 {
        self.size().1
    }

    /// Возвращает размер ячейки, которая содержится в прямоугольнике.
    #[inline]
    fn cell_len(&self) -> usize {
        (self.row() * self.column() + 1) as usize
    }

    /// Возвращает True, если точка находится в прямоугольнике.
    #[inline]
    fn contains(&self, p: Point) -> bool {
        let size = self.size();
        0 <= p.0 && p.0 < size.0 && 0 <= p.1 && p.1 < size.1
    }

    /// Преобразовать точку в соответствующую идентификатору соты.
    #[inline]
    fn point_to_cellid(&self, p: Point) -> CellId {
        if self.contains(p) {
            CellId::new((p.0 * self.column() + p.1 + 1) as usize)
        } else {
            CELL_ID_OUTSIDE
        }
    }
}

/// Карта мира. Храним объекты в нем.
pub struct Map<T> {
    size: Size,
    data: Vec<T>,
}

impl<T> Map<T> {
    /// Создаем мир/поле/карту с данными.
    #[inline]
    pub fn new(size: Size, outside: T, mut data: Vec<T>) -> Map<T> {
        assert_eq!((size.0 * size.1) as usize, data.len());
        data.insert(0, outside);
        Map {
            size: size,
            data: data,
        }
    }

    /// Создаем новый, пустой мир/поле/карту.
    #[inline]
    pub fn new_empty(size: Size, outside: T, init: T) -> Map<T>
        where T: Clone
    {
        let data = vec![init; (size.0 * size.1) as usize];
        Map::new(size, outside, data)
    }
}

impl<T> Geom for Map<T> {
    #[inline]
    fn size(&self) -> Size {
        self.size
    }
}

/// Получаем значение по координатам.
impl<T> Index<Point> for Map<T> {
    type Output = T;

    #[inline]
    fn index(&self, p: Point) -> &T {
        let idx = self.point_to_cellid(p).id();
        &self.data[idx]
    }
}

/// Получаем мутабельное значение по координатам.
impl<T> IndexMut<Point> for Map<T> {
    #[inline]
    fn index_mut(&mut self, p: Point) -> &mut T {
        let idx = self.point_to_cellid(p).id();
        &mut self.data[idx]
    }
}