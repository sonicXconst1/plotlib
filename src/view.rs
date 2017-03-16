/*!
The `view` module provides structures for showing data in various ways.
*/

use std;
use std::f64;

use svg;
use svg::Node;

use representation::Representation;
use axis;
use svg_render;
use text_render;

/// Standard 1-dimensional view with a continuous x-axis
pub struct View<'a> {
    pub representations: Vec<&'a Representation>,
    x_range: Option<axis::Range>,
    y_range: Option<axis::Range>,
}

impl<'a> View<'a> {
    /**
    Create an empty view
    */
    pub fn new() -> View<'a> {
        View {
            representations: vec![],
            x_range: None,
            y_range: None,
        }
    }

    /**
    Add a representation to the view
    */
    pub fn add(mut self, repr: &'a Representation) -> Self {
        self.representations.push(repr);
        self
    }

    /**
    Set the x range for the view
    */
    pub fn x_range(mut self, min: f64, max: f64) -> Self {
        self.x_range = Some(axis::Range::new(min, max));
        self
    }

    /**
    Set the y range for the view
    */
    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some(axis::Range::new(min, max));
        self
    }

    fn default_x_range(&self) -> axis::Range {
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        for repr in self.representations.iter() {
            let (this_x_min, this_x_max) = repr.range(0);
            x_min = x_min.min(this_x_min);
            x_max = x_max.max(this_x_max);
        }
        axis::Range::new(x_min, x_max)
    }

    fn default_y_range(&self) -> axis::Range {
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for repr in self.representations.iter() {
            let (this_y_min, this_y_max) = repr.range(1);
            y_min = y_min.min(this_y_min);
            y_max = y_max.max(this_y_max);
        }
        axis::Range::new(y_min, y_max)
    }

    /**
    Create an SVG rendering of the view
    */
    pub fn to_svg(&self, face_width: f64, face_height: f64) -> svg::node::element::Group {
        let mut view_group = svg::node::element::Group::new();

        let default_x_range = self.default_x_range();
        let x_range = self.x_range.as_ref().unwrap_or(&default_x_range);

        let default_y_range = self.default_y_range();
        let y_range = self.y_range.as_ref().unwrap_or(&default_y_range);

        let x_axis = axis::Axis::new(x_range.lower, x_range.upper);
        let y_axis = axis::Axis::new(y_range.lower, y_range.upper);

        // Then, based on those ranges, draw each repr as an SVG
        for repr in self.representations.iter() {
            let repr_group = repr.to_svg(&x_axis, &y_axis, face_width, face_height);
            view_group.append(repr_group);
        }

        // Add in the axes
        view_group.append(svg_render::draw_x_axis(&x_axis, face_width));
        view_group.append(svg_render::draw_y_axis(&y_axis, face_height));
        view_group
    }

    /**
    Create a text rendering of the view
    */
    pub fn to_text(&self, face_width: u32, face_height: u32) -> String {
        let default_x_range = self.default_x_range();
        let x_range = self.x_range.as_ref().unwrap_or(&default_x_range);

        let default_y_range = self.default_y_range();
        let y_range = self.y_range.as_ref().unwrap_or(&default_y_range);

        let x_axis = axis::Axis::new(x_range.lower, x_range.upper);
        let y_axis = axis::Axis::new(y_range.lower, y_range.upper);

        let (y_axis_string, longest_y_label_width) =
            text_render::render_y_axis_strings(&y_axis, face_height);

        let (x_axis_string, start_offset) = text_render::render_x_axis_strings(&x_axis, face_width);

        let left_gutter_width = std::cmp::max(longest_y_label_width as i32 + 1,
                                              start_offset.wrapping_neg()) as
                                u32;

        let view_width = face_width + 1 + left_gutter_width + 1;
        let view_height = face_height + 3;

        let blank: Vec<String> =
            (0..view_height).map(|_| (0..view_width).map(|_| ' ').collect()).collect();
        let mut view_string = blank.join("\n");

        for repr in self.representations.iter() {
            let face_string = repr.to_text(&x_axis, &y_axis, face_width, face_height);
            view_string =
                text_render::overlay(&view_string, &face_string, left_gutter_width as i32 + 1, 0);
        }

        let view_string = text_render::overlay(&view_string,
                                               &y_axis_string,
                                               left_gutter_width as i32 - 1 -
                                               longest_y_label_width,
                                               0);
        let view_string = text_render::overlay(&view_string,
                                               &x_axis_string,
                                               left_gutter_width as i32 + 0,
                                               face_height as i32 + 0);

        view_string
    }
}