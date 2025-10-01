
use uuid::Uuid;
use rgb::RGB8;

//rgb info: https://crates.io/crates/rgb

pub struct Canvas 
{
    //The canvases Id
    id: Uuid,
    //If I had more time, I would use a 1d vector, and treat it like a matrix, and them implement row and col operations to pretend like its 2d.
    //But this is mostly fine. I don't do validity checks past the first row and assume that it is actually an n * m matrix.
    values: Vec<Vec<RGB8>>, //The canvas Values
    rows: usize,
    cols: usize,

} 

impl Canvas
{

    pub const DEFAULT_ROWS: usize = 1000;
    pub const DEFAULT_COLS: usize = 1000;
    pub const DEFAULT_COLOR: RGB8 = RGB8::new(255, 255, 255); //default value is white.



    /*
    id: The id for the canvas
    values: a vector representing 
     */
    pub fn new(id: Uuid, values: Vec<Vec<RGB8>>, rows: usize, cols: usize) -> Self
    {
        //The input 2d vector cannot be empty.
        assert_ne!(values.len(), 0, "Input vector cannot be empty, no rows found");
        assert_ne!(values[0].len(), 0, "Input vector cannot be empty, no cols found in first row");
        //The input rows and cols values must be equal to the size of the vector
        assert_eq!(rows * cols, values.len() * values[0].len());
        Self {
            id: id,
            values: values,
            rows: rows,
            cols: cols,
        }
    }


    pub fn id(&self) -> Uuid
    {
        self.id
    }


    pub fn rows(&self) -> usize
    {
        self.rows
    }

    pub fn cols(&self) -> usize
    {
        self.cols
    }

    pub fn get_color(&self, row: usize, cols: usize) -> RGB8
    {
        self.values[row][cols]
    }

    pub fn set_color(& mut self, row: usize, cols: usize, color: RGB8)
    {
        self.values[row][cols] = color;
    }

    pub fn create_blank_canvas(rows: usize, cols: usize) -> Vec<Vec<RGB8>>
    {
        let mut new_rows: Vec<Vec<RGB8>> = Vec::with_capacity(rows);
        for row in 0..rows
        {
            let mut new_col: Vec<RGB8> = Vec::with_capacity(cols);
            for col in 0..cols
            {
                new_col.push(Canvas::DEFAULT_COLOR.clone());
            }
            new_rows.push(new_col);
        }

        new_rows
        
    }

}

