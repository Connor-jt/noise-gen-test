import("../pkg/index.js").catch(console.error);

//  RANDOM PIXELS TEST

// const canvas = document.querySelector("#canvas")
// //const ctx = document.querySelector("#canvas").getContext("2d");

// //const size = 20;
// //ctx.canvas.width = size;
// //ctx.canvas.height = size;

// const img = new ImageData(canvas.width, canvas.height)
// const img_data = new Uint32Array(img.data.buffer)


// var myPixels = []
// for (var i=1; i<=10; i++) 
//   myPixels.push({
//     x: Math.floor(Math.random() * canvas.width), 
//     y: Math.floor(Math.random() * canvas.height)
//   })
// myPixels.forEach((p) => img_data[p.y * canvas.width + p.x] = 4294967295)
// canvas.getContext("2d").putImageData(img, 0, 0)


// FILL RECTS TEST


// console.log("test")


// const drawRect = (x, y, w=1, h=1, color="#fff") => {
//     ctx.fillStyle = color;
//     ctx.fillRect(x, y, w, h);
// };


// const size = 200;
// const ctx = document.querySelector("#canvas").getContext("2d");

// ctx.canvas.width = size;
// ctx.canvas.height = size;



// // 1. Draw black background
// drawRect(0, 0, size, size, "#000");
// drawRect(13, 2);
// drawRect(5, 1);
// drawRect(2, 3);
// drawRect(7, 15);