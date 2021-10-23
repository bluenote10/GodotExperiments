const fs = require('fs');

function convert_output(polygons) {
  let output = [];

  console.log(polygons);
  for (let subpolygons of polygons) {
    for (let subpolygon of subpolygons) {
      console.log(JSON.stringify(subpolygon));
      subpolygon.push(subpolygon[0])
      let output_polygon = {
        exterior: subpolygon.map(point => { return {x: point[0], y: point[1]}}),
        interiors: [],
      };
      output.push(output_polygon);
    }
  }

  return output;
}


function store_polygons(polygons, filename) {
  let output = convert_output(polygons);
  fs.writeFileSync(filename, JSON.stringify(output));
}


module.exports = { convert_output, store_polygons };