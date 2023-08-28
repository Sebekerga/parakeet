/// Enum for page margin direction
pub enum PageMargin {
    Top,
    Bottom,
    Left,
    Right,
}

/// Struct for page properties, passed to the rendering engine
pub struct PageProperties {
    /// Width of the paper in inches
    pub paper_width: f64,
    /// Height of the paper in inches
    pub paper_height: f64,
    /// Top margin in inches
    pub margin_top: f64,
    /// Bottom margin in inches
    pub margin_bottom: f64,
    /// Left margin in inches
    pub margin_left: f64,
    /// Right margin in inches
    pub margin_right: f64,
}

impl PageProperties {
    /// Create a new PagePropertiesBuilder
    pub fn builder() -> PagePropertiesBuilder {
        PagePropertiesBuilder::default()
    }
}

/// Builder for PageProperties
pub struct PagePropertiesBuilder {
    paper_width: f64,
    paper_height: f64,
    margin_top: f64,
    margin_bottom: f64,
    margin_left: f64,
    margin_right: f64,
}

impl Default for PagePropertiesBuilder {
    fn default() -> Self {
        Self {
            paper_width: 1.0,
            paper_height: 1.0,
            margin_top: 0.0,
            margin_bottom: 0.0,
            margin_left: 0.0,
            margin_right: 0.0,
        }
    }
}

impl PagePropertiesBuilder {
    /// Build the PageProperties, consuming the builder
    pub fn build(self) -> PageProperties {
        PageProperties {
            paper_width: self.paper_width,
            paper_height: self.paper_height,
            margin_top: self.margin_top,
            margin_bottom: self.margin_bottom,
            margin_left: self.margin_left,
            margin_right: self.margin_right,
        }
    }

    /// Create a new PagePropertiesBuilder from the given width and height
    /// # Arguments
    /// * `width` - The width in inches
    /// * `height` - The height in inches
    pub fn from_size(width: f64, height: f64) -> Self {
        PagePropertiesBuilder {
            paper_width: width,
            paper_height: height,
            margin_top: 0.0,
            margin_bottom: 0.0,
            margin_left: 0.0,
            margin_right: 0.0,
        }
    }

    /// Set page width for the resulting paper page
    /// # Arguments
    /// * `width` - The width in inches
    pub fn set_paper_width(&mut self, width: f64) -> &mut Self {
        self.paper_width = width;
        self
    }

    /// Set page height for the resulting paper page
    /// # Arguments
    /// * `height` - The height in inches
    pub fn set_paper_height(&mut self, height: f64) -> &mut Self {
        self.paper_height = height;
        self
    }

    /// Set page margin for the resulting paper page
    /// # Arguments
    /// * `direction` - The direction of the margin
    /// * `margin` - The margin in inches
    pub fn set_margin(&mut self, direction: PageMargin, margin: f64) -> &mut Self {
        match direction {
            PageMargin::Top => self.margin_top = margin,
            PageMargin::Bottom => self.margin_bottom = margin,
            PageMargin::Left => self.margin_left = margin,
            PageMargin::Right => self.margin_right = margin,
        }
        self
    }
}
