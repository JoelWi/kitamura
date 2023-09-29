# Kitamura
Kitamura is an HTML templating engine for rendering HTML based on placeholders
defined in your HTML. Placeholders are provided in the form of content parameters
that are in JSON.
# Overview
Below is the general idea of how to use Kitamura. Formatting of the input HTML
is respected, and overall there are no expectations that anything but what Kitamura
is looking for will be modified.
```text
Input HTML
<html>Hello ${first_name}!</html>

Input Data
{
  "first_name": "Joel"
}

Output HTML
<html>Hello Joel!</html>
```

# Features
Variables

Lists

# Examples
```
use std::collections::HashMap;
use kitamura::render_template;
use serde_json::json;

let input_html = "<html>Hello ${first_name}!</html>";
let mut input_data = HashMap::new();
input_data.insert("first_name".to_string(), json!("Joel"));

let output_html = render_template(input_html.to_string(), input_data);
assert_eq!(output_html, "<html>Hello Joel!</html>");
```
```
use std::collections::HashMap;
use kitamura::render_template;
use serde_json::json;

let input_html =
"<html>
 <ul>
   {#for fruit in fruits#}
   <ul>
     <li>${fruit.name}</li>
     <li>${fruit.colour}</li>
     <li>${fruit.weight}</li>
   </ul>
   {#endfor#}
 </ul>
</html>";
let mut input_data = HashMap::new();
input_data.insert(
fruits".to_string(),
son!([{"name": "Lemon", "colour": "Yellow", "weight": "150g"},
"name": "shiikuwasha", "colour": "Green", "weight": "80g"},
"name": "Lychee", "colour": "Red", "weight": "50g"}]),
;

let output_html = render_template(input_html.to_string(), input_data);
assert_eq!(output_html,
"<html>
 <ul>
   <ul>
     <li>Lemon</li>
     <li>Yellow</li>
     <li>150g</li>
   </ul>
   <ul>
     <li>shiikuwasha</li>
     <li>Green</li>
     <li>80g</li>
   </ul>
   <ul>
     <li>Lychee</li>
     <li>Red</li>
     <li>50g</li>
   </ul>
 </ul>
</html>");
```