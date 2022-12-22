/*
Given data (a dictionary of trends), returns an svg element

data must be in the following format:
{
  "cat": [
    1990, 0.000003, 1992, 0.000004, 1996, 0.0000034, 2040, 0.0000078, 2042, 0.0000073
  ],
  "dog": [
    1990, 0.00003, 1992, 0.00004, 1996, 0.000034, 2040, 0.000078, 2042, 0.000073
  ]
}

where in each trend, the years and values are interleaved in one array
trends must be in temporal order
*/
export function generate_chart(data) {
  let min_year = Infinity;
  let max_year = -Infinity;
  let max_value = -Infinity;
  for (let k in data) {
    if (!Array.isArray(data[k]) || data[k].length % 2 != 0) {
      console.error("data is improperly formatted");
      return document.createTextNode("data is improperly formatted");
    }
    for (let a = 0; a < data[k].length; a += 2) {
      min_year = Math.min(min_year, data[k][a]);
      max_year = Math.max(max_year, data[k][a]);
      max_value = Math.max(max_value, data[k][a + 1]);
    }
  }

  if (max_value == -Infinity) {
    min_year = 0;
    max_year = 1;
    max_value = 1;
  } else if (min_year == max_year) {
    min_year -= 1;
    max_year += 1;
  }

  let bounds = new Bounds(190, 370, 1200, 10, min_year, max_year, 0, max_value);
  const view_box_width = 1400;
  const view_box_height = 500;

  let chart = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  chart.setAttribute("viewBox", `0 0 ${view_box_width} ${view_box_height}`);
  chart.classList.add("graph");
  chart.innerHTML = `
    <g class="grid">
      <line x1="${bounds.X1}" x2="${bounds.X1}" y1="${bounds.Y2}" y2="${bounds.Y1}"></line>
    </g>
    <g class="grid">
      <line x1="${bounds.X1}" x2="${bounds.X2}" y1="${bounds.Y1}" y2="${bounds.Y1}"></line>
    </g>
    <g class="grid">
        <line class="infoline" visibility="hidden"></line>
    </g>
    `;

  let stuff = [];
  let c = 0;
  for (let k in data) {
    if (data[k].length > 0) {
      let color = TREND_COLORS[c++ % TREND_COLORS.length];
      stuff.push(generate_polyline(data[k], bounds, color));
      stuff.push(
        generate_trend_label(bounds, k, data[k][data[k].length - 1], color)
      );
    }
  }
  stuff = stuff.concat(generate_xlabels(bounds));
  stuff = stuff.concat(generate_ylabels(bounds));
  for (let a = 0; a < stuff.length; a++) {
    chart.appendChild(stuff[a]);
  }

  let infogroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  infogroup.setAttribute("visibility", "hidden");
  infogroup.classList.add("infogroup");
  chart.appendChild(infogroup);

  chart.addEventListener("mousemove", (e) => {
    let x_scale = view_box_width / chart.clientWidth;
    let y_scale = view_box_height / chart.clientHeight;
    let mx = e.offsetX * x_scale;
    let my = e.offsetY * y_scale;

    if (
      mx <= bounds.X1 ||
      mx >= bounds.X2 ||
      my <= bounds.Y2 ||
      my >= bounds.Y1
    ) {
      chart.querySelector(".infogroup").setAttribute("visibility", "hidden");
      chart.querySelector(".infoline").setAttribute("visibility", "hidden");
      return;
    }
    let group = chart.querySelector(".infogroup");
    let line = chart.querySelector(".infoline");
    group.setAttribute("visibility", "visible");
    line.setAttribute("visibility", "visible");
    line.setAttribute("x1", mx);
    line.setAttribute("x2", mx);
    line.setAttribute("y1", bounds.Y1);
    line.setAttribute("y2", bounds.Y2);
    let year = Math.round((mx - bounds.X1) / bounds.x_scale) + bounds.min_year;
    let step = getRoundBound((bounds.max_value - bounds.min_value) / 16);
    let precision = Math.max(0, 1 - Math.floor(Math.log10(step)));
    let hd = get_hover_data(data, year, precision);
    modify_infobox(group, mx, my, mx < view_box_width * 0.65, x_scale, hd);
  });

  return chart;
}

const HOVER_RESULT_LIMIT = 50;
const TREND_COLORS = [
  "#1976d2",
  "#d32f2f",
  "#2e7d32",
  "#f57c00",
  "#0d47a1",
  "#42a5f5",
  "#880e4f",
];

class Bounds {
  constructor(X1, Y1, X2, Y2, min_year, max_year, min_value, max_value) {
    this.X1 = X1;
    this.Y1 = Y1;
    this.X2 = X2;
    this.Y2 = Y2;
    this.min_year = min_year;
    this.max_year = max_year;
    this.min_value = min_value;
    this.max_value = max_value;
    this.x_scale = (X2 - X1) / (max_year - min_year);
    this.y_scale = (Y2 - Y1) / (max_value - min_value);
  }

  x_pos(year) {
    return this.X1 + this.x_scale * (year - this.min_year);
  }

  y_pos(value) {
    return this.Y1 + this.y_scale * (value - this.min_value);
  }
}

function generate_xlabels(bounds) {
  let step = 1;
  if (bounds.max_year - bounds.min_year > 16) {
    step = getRoundBound((bounds.max_year - bounds.min_year) / 16);
  }
  let year = Math.ceil(bounds.min_year / step) * step;

  let tg = document.createElementNS("http://www.w3.org/2000/svg", "g");
  tg.classList.add("grid");
  let lg = document.createElementNS("http://www.w3.org/2000/svg", "g");
  lg.classList.add("labels");
  while (year <= bounds.max_year) {
    let x_pos = bounds.x_pos(year);
    let tick = document.createElementNS("http://www.w3.org/2000/svg", "line");
    tick.setAttribute("x1", x_pos);
    tick.setAttribute("x2", x_pos);
    tick.setAttribute("y1", bounds.Y1);
    tick.setAttribute("y2", bounds.Y1 + 10);
    tg.appendChild(tick);

    let label = document.createElementNS("http://www.w3.org/2000/svg", "text");
    label.setAttribute("x", x_pos - 15);
    label.setAttribute("y", bounds.Y1 + 30);
    label.appendChild(document.createTextNode(year.toString()));

    lg.appendChild(label);
    year += step;
  }

  return [tg, lg];
}

function generate_ylabels(bounds) {
  let step = getRoundBound((bounds.max_value - bounds.min_value) / 16);
  let precision = Math.max(0, 1 - Math.floor(Math.log10(step)));
  let value = Math.ceil(bounds.min_value / step) * step;

  let tg = document.createElementNS("http://www.w3.org/2000/svg", "g");
  tg.classList.add("grid");
  let lg = document.createElementNS("http://www.w3.org/2000/svg", "g");
  lg.classList.add("labels");
  while (value <= bounds.max_value) {
    let y_pos = bounds.y_pos(value);
    let tick = document.createElementNS("http://www.w3.org/2000/svg", "line");
    tick.setAttribute("x1", bounds.X1 - 10);
    tick.setAttribute("x2", bounds.X1);
    tick.setAttribute("y1", y_pos);
    tick.setAttribute("y2", y_pos);
    tg.appendChild(tick);

    let label = document.createElementNS("http://www.w3.org/2000/svg", "text");
    label.setAttribute("x", bounds.X1 - 80);
    label.setAttribute("y", y_pos + 5);
    label.appendChild(document.createTextNode(value.toFixed(precision)));

    lg.appendChild(label);
    value += step;
  }

  return [tg, lg];
}

function generate_trend_label(bounds, word, value, color) {
  let lg = document.createElementNS("http://www.w3.org/2000/svg", "g");
  lg.classList.add("labels");
  let label = document.createElementNS("http://www.w3.org/2000/svg", "text");
  label.setAttribute("x", bounds.X2 + 5);
  label.setAttribute("y", bounds.y_pos(value) + 5);
  label.setAttribute("fill", color);
  label.appendChild(document.createTextNode(word));
  lg.appendChild(label);
  return lg;
}

/*
Returns a polyline (trendline) element for svg
*/
function generate_polyline(data_array, bounds, color) {
  let points = [];
  for (let a = 0; a < data_array.length; a += 2) {
    let x = bounds.x_pos(data_array[a]);
    let y = bounds.y_pos(data_array[a + 1]);
    points.push(`${x},${y}`);
  }

  let newline = document.createElementNS(
    "http://www.w3.org/2000/svg",
    "polyline"
  );
  newline.setAttribute("fill", "none");
  newline.setAttribute("stroke", color);
  newline.setAttribute("stroke-width", "2");
  newline.setAttribute("points", points.join(" "));
  return newline;
}

function get_hover_data(data, year, precision) {
  let res = [];
  let c = 0;
  for (let k in data) {
    if (data[k].length > 0) {
      let color = TREND_COLORS[c++ % TREND_COLORS.length];
      res.push([color, k, bin_search(data[k], year)]);
    }
  }
  res.sort((a, b) => Math.sign(b[2] - a[2])); // sort (reverse) by value at year
  for (let row of res) {
    row[2] = row[2].toFixed(precision); // convert float to string
  }

  return [year].concat(res.slice(0, HOVER_RESULT_LIMIT));
}

function bin_search(ar, year) {
  let min = 0;
  let max = ar.length / 2 - 1;
  while (min <= max) {
    let mid = Math.floor((min + max) / 2);
    if (ar[2 * mid] == year) {
      return ar[2 * mid + 1];
    } else if (ar[2 * mid] > year) {
      max = mid - 1;
    } else {
      min = mid + 1;
    }
  }
  return 0;
}

function modify_infobox(infogroup, mx, my, place_right, x_scale, hover_data) {
  let infobox = document.createElementNS("http://www.w3.org/2000/svg", "rect");
  infogroup.replaceChildren(infobox);
  infobox.setAttribute("rx", "3");
  infobox.setAttribute("fill", "white");
  infobox.classList.add("infobox");
  infobox.setAttribute("width", "250");
  infobox.setAttribute("height", 15 + 20 * hover_data.length);
  infobox.setAttribute("y", my);

  let stx = mx + 15;
  if (!place_right) {
    let w = infobox.getBoundingClientRect().width * x_scale;
    stx = mx - w - 15;
  }

  infobox.setAttribute("x", stx);

  let line = document.createElementNS("http://www.w3.org/2000/svg", "text");
  line.appendChild(document.createTextNode(hover_data[0]));
  line.setAttribute("x", stx + 10);
  line.setAttribute("y", my + 20);
  infogroup.appendChild(line);

  for (let a = 1; a < hover_data.length; a++) {
    let circ = document.createElementNS("http://www.w3.org/2000/svg", "circle");
    circ.setAttribute("cx", stx + 15);
    circ.setAttribute("cy", my + 20 * a + 20);
    circ.setAttribute("r", 5);
    circ.setAttribute("fill", hover_data[a][0]);

    let label = document.createElementNS("http://www.w3.org/2000/svg", "text");
    let text = hover_data[a][1] + ": " + hover_data[a][2];
    label.setAttribute("x", stx + 25);
    label.setAttribute("y", my + 20 * a + 25);
    label.appendChild(document.createTextNode(text));

    infogroup.appendChild(circ);
    infogroup.appendChild(label);
  }
}

/*
Computes a "round number" r such that
num < r
*/
function getRoundBound(num) {
  let good = [2, 5, 10];
  let scale = Math.pow(10, Math.floor(Math.log10(num)));
  let a = 0;
  while (num / scale > good[a]) a++;
  return good[a] * scale;
}
