const fs = require('fs');

function convert_output(polygons) {
  let output = [];

  console.log(polygons);
  for (let polygon of polygons) {
    console.log(polygon["regions"]);
    polygon["regions"].forEach(region => {
      region.push(region[0])
      let output_polygon = {
        exterior: region.map(point => { return {x: point[0], y: point[1]}}),
        interiors: [],
      };
      output.push(output_polygon);
    })
  }

  return output;
}


function store_polygons(polygons, filename) {
  let output = convert_output(polygons);
  fs.writeFileSync(filename, JSON.stringify(output));
}


module.exports = { convert_output, store_polygons };