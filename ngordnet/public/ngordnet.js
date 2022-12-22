import {generate_chart} from "./graph.js";


function showTextResult(text) {
    let pre = document.createElement("pre");
    pre.innerText = text
    document.getElementById("main").replaceChildren(pre);
}

function showGraphResult(data) {
    document.getElementById("main").replaceChildren(generate_chart(data));
}



let form = document.getElementById("main-form");
form.onsubmit = function(e) {
    e.preventDefault();
    let queryType = form.queryType.value;
    let words = encodeURIComponent(form.words.value);
    let startYear = encodeURIComponent(form.startYear.value);
    let endYear = encodeURIComponent(form.endYear.value);
    let k = encodeURIComponent(form.k.value);
    let queryString = `?words=${words}&start-year=${startYear}&end-year=${endYear}&k=${k}`;

    showTextResult("loading...");

    if (queryType == "History (graph)") {
        fetch("history" + queryString)
            .then((r) => r.json())
            .then(showGraphResult)
            .catch((e) => alert("error! " + e))
    } else if (queryType == "History (text)") {
        fetch("history" + queryString)
            .then((r) => r.text())
            .then(showTextResult)
            .catch((e) => alert("error! " + e))
    } else if (queryType == "Synonyms (text)") {
        fetch("synonyms" + queryString)
            .then((r) => r.text())
            .then(showTextResult)
            .catch((e) => alert("error! " + e))
    } else if (queryType == "Hyponyms (text)") {
        fetch("hyponyms" + queryString)
            .then((r) => r.text())
            .then(showTextResult)
            .catch((e) => alert("error! " + e))
    }
}
