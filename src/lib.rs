//! # Kitamura
//! Kitamura is an HTML templating engine for rendering HTML based on placeholders
//! defined in your HTML. Placeholders are provided in the form of content parameters
//! that are in JSON.
//!
//! # Overview
//! Below is the general idea of how to use Kitamura. Formatting of the input HTML
//! is respected, and overall there are no expectations that anything but what Kitamura
//! is looking for will be modified.
//!
//! Kitamura will return a Result, which will contain either the rendered HTML, or
//! an error message.
//! ```text
//! Input HTML
//! <html>Hello ${first_name}!</html>
//!
//! Input Data
//! {
//!   "first_name": "Joel"
//! }
//!
//! Output HTML
//! <html>Hello Joel!</html>
//! ```
//!
//! # Features
//! Variables
//!
//! Lists
//!
//! # Examples
//! ```
//! use std::collections::HashMap;
//! use kitamura::render_template;
//! use serde_json::json;
//!
//! let input_html = "<html>Hello ${first_name}!</html>";
//! let mut input_data = HashMap::new();
//! input_data.insert("first_name".to_string(), json!("Joel"));
//!
//! let output_html = render_template(input_html.to_string(), input_data).unwrap();
//! assert_eq!(output_html, "<html>Hello Joel!</html>");
//! ```
//! ```
//! use std::collections::HashMap;
//! use kitamura::render_template;
//! use serde_json::json;
//!
//! let input_html =
//! "<html>
//!  <ul>
//!    {#for fruit in fruits#}
//!    <ul>
//!      <li>${fruit.name}</li>
//!      <li>${fruit.colour}</li>
//!      <li>${fruit.weight}</li>
//!    </ul>
//!    {#endfor#}
//!  </ul>
//! </html>";
//! let mut input_data = HashMap::new();
//! input_data.insert(
//!"fruits".to_string(),
//!json!([{"name": "Lemon", "colour": "Yellow", "weight": "150g"},
//!{"name": "shiikuwasha", "colour": "Green", "weight": "80g"},
//!{"name": "Lychee", "colour": "Red", "weight": "50g"}]),
//!);
//!
//! let output_html = render_template(input_html.to_string(), input_data).unwrap();
//! assert_eq!(output_html,
//! "<html>
//!  <ul>
//!    <ul>
//!      <li>Lemon</li>
//!      <li>Yellow</li>
//!      <li>150g</li>
//!    </ul>
//!    <ul>
//!      <li>shiikuwasha</li>
//!      <li>Green</li>
//!      <li>80g</li>
//!    </ul>
//!    <ul>
//!      <li>Lychee</li>
//!      <li>Red</li>
//!      <li>50g</li>
//!    </ul>
//!  </ul>
//!</html>");
//! ```
//! ```
//! use std::collections::HashMap;
//! use kitamura::render_template;
//! use serde_json::json;
//!
//!let input_html =
//!"<html>
//!  <ul>
//!  {#for continent in continents#}
//!    <li>${continent.name}</li>
//!    <ul>
//!    {#for country in continent.countries#}
//!      <li>${country.name}</li>
//!      <ul>
//!      {#for city in country.cities#}
//!        <li>
//!          <ul>
//!            <li>${city.name}</li>
//!            <li>${city.time}</li>
//!            <li>${city.tempurature}</li>
//!          <ul>
//!        </li>
//!      {#endfor#}
//!      </ul>
//!    {#endfor#}
//!    </ul>
//!  {#endfor#}
//!  </ul>
//!</html>";
//! let mut input_data = HashMap::new();
//! input_data.insert(
//!"continents".to_string(),
//!json!([{"name":"Oceania","countries":[{"name":"Australia","cities":[{"name":"Brisbane","time":"9:26PM","tempurature":"14C"},{"name":"Melbourne","time":"9:26PM","tempurature":"14C"},{"name":"Adelaide","time":"8:56PM","tempurature":"15C"}]},{"name":"New Zealand","cities":[{"name":"Wellington","time":"11:26PM","tempurature":"12C"}]}]},{"name":"Europe","countries":[{"name":"England","cities":[{"name":"Manchester","time":"12:26PM","tempurature":"16C"},{"name":"London","time":"12:26PM","tempurature":"23C"}]}]}]),
//!);
//!
//!let expected_rendered_output = "<html>
//!  <ul>
//!    <li>Oceania</li>
//!    <ul>
//!      <li>Australia</li>
//!      <ul>
//!        <li>
//!          <ul>
//!            <li>Brisbane</li>
//!            <li>9:26PM</li>
//!            <li>14C</li>
//!          <ul>
//!        </li>
//!        <li>
//!          <ul>
//!            <li>Melbourne</li>
//!            <li>9:26PM</li>
//!            <li>14C</li>
//!          <ul>
//!        </li>
//!        <li>
//!          <ul>
//!            <li>Adelaide</li>
//!            <li>8:56PM</li>
//!            <li>15C</li>
//!          <ul>
//!        </li>
//!      </ul>
//!      <li>New Zealand</li>
//!      <ul>
//!        <li>
//!          <ul>
//!            <li>Wellington</li>
//!            <li>11:26PM</li>
//!            <li>12C</li>
//!          <ul>
//!        </li>
//!      </ul>
//!    </ul>
//!    <li>Europe</li>
//!    <ul>
//!      <li>England</li>
//!      <ul>
//!        <li>
//!          <ul>
//!            <li>Manchester</li>
//!            <li>12:26PM</li>
//!            <li>16C</li>
//!          <ul>
//!        </li>
//!        <li>
//!          <ul>
//!            <li>London</li>
//!            <li>12:26PM</li>
//!            <li>23C</li>
//!          <ul>
//!        </li>
//!      </ul>
//!    </ul>
//!  </ul>
//!</html>";
//!
//! let output_html = render_template(input_html.to_string(), input_data).unwrap();
//! assert_eq!(output_html, expected_rendered_output);
//! ```

use std::collections::HashMap;

mod ast;
mod template;
mod token;

pub fn render_template(
    html: String,
    parameters: HashMap<String, serde_json::Value>,
) -> Result<String, String> {
    template::render_template(html, parameters)
}
