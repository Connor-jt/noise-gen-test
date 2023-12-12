
import("../pkg/index.js").catch(console.error);
import init, { redraw_canvas } from '../pkg/index_bg.js';

function update_values(){
    x_elem.innerText = x.toString();
    y_elem.innerText = y.toString();
    z_elem.innerText = z.toString();

    freq_elem.innerText = frequency.toString();
    octs_elem.innerText = octaves.toString();
    lacu_elem.innerText = lacunarity.toString();
    pers_elem.innerText = persistence.toString();

    seed_elem.innerText = seed.toString();
}


var x = 0.0;
var y = 0.0;
var z = 0.0;

var x_elem = document.getElementById("x");
var y_elem = document.getElementById("y");
var z_elem = document.getElementById("z");
var x_slider_elem = document.getElementById("x_slider");
var y_slider_elem = document.getElementById("y_slider");
var z_slider_elem = document.getElementById("z_slider");

var frequency = 0.005;
var octaves = 4.0;
var lacunarity = 2.0;
var persistence = 0.5;

var freq_elem = document.getElementById("freq");
var octs_elem = document.getElementById("octs");
var lacu_elem = document.getElementById("lacu");
var pers_elem = document.getElementById("pers");
var freq_slider_elem = document.getElementById("freq_slider");
var octs_slider_elem = document.getElementById("octs_slider");
var lacu_slider_elem = document.getElementById("lacu_slider");
var pers_slider_elem = document.getElementById("pers_slider");

var seed = 35;

var seed_elem = document.getElementById("seed");
var seed_slider_elem = document.getElementById("seed_slider");

var time_elem = document.getElementById("time_span");

function reload_noise(){
    console.log("values updated !!")
    update_values();
    // then time how long it takes
    var start = performance.now();
    redraw_canvas(x, y, z, frequency, octaves, lacunarity, persistence, seed);
    var end = performance.now();
    time_elem.innerText = (end - start).toString() + " ms";
}

function change_crdx() {x = x_slider_elem.value; reload_noise();}
function change_crdy() {y = y_slider_elem.value; reload_noise();}
function change_crdz() {z = z_slider_elem.value; reload_noise();}

function change_freq() {frequency = freq_slider_elem.value; reload_noise();}
function change_octs() {octaves = octs_slider_elem.value; reload_noise();}
function change_lacu() {lacunarity = lacu_slider_elem.value; reload_noise();}
function change_pers() {persistence = pers_slider_elem.value; reload_noise();}

function change_seed() {seed = seed_slider_elem.value; reload_noise();}



update_values(); // run our init to fix up display values
// + init function bidnings
//document.getElementById("x_slider").addEventListener('input', function(){change_crdx(this.value);}, true);
x_slider_elem.oninput = change_crdx;
y_slider_elem.oninput = change_crdy;
z_slider_elem.oninput = change_crdz;

freq_slider_elem.oninput = change_freq;
octs_slider_elem.oninput = change_octs;
lacu_slider_elem.oninput = change_lacu;
pers_slider_elem.oninput = change_pers;

seed_slider_elem.oninput = change_seed;


