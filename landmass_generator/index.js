// Init
let itterations = document.getElementById("itterations");
let splats = document.getElementById("splats");
let initialRadius = document.getElementById("initialRadius");

const canvas = document.getElementById("canvas");
const width = canvas.width;
const height = canvas.height;
const context = canvas.getContext("2d");
const image = context.getImageData(0, 0, width, height);
let data = image.data;

function run() {
    const points = [];
    for (let i = 0; i < itterations.value; i++) {
        for (let j = 0; j < splats.value; j++) {
            points.push({
                x: (Math.random() * 700) + 150,
                y: (Math.random() * 700) + 150,
                radius: radius(i)
            });
        }

        if (i % 10 == 0) {
            console.log(i);
        }
    }

    render(points);
}

function render(points) {
    for (let x = 0; x < width; x++) {
        for (y = 0; y < height; y++) {
            setRGBA(x, y, 0, 0, 255, 255);
            for (point of points) {
                if (inside(point, x, y)) {
                    setRGBA(x, y, 0, 255, 0, 255);
                }
            }
        }
        if (x % 100 == 0) {
            console.log(Math.floor(x / 100) / 10);
        }
    }
    context.putImageData(image, 0, 0);
}

function inside(point, x, y) {
    let difX = point.x - x;
    let difY = point.y - y;
    return Math.sqrt(difX * difX + difY * difY) <= point.radius;
}

function setRGBA(x, y, r, g, b, a) {
    const index = ((y * width) + x) * 4;
    data[index] = r;
    data[index + 1] = g;
    data[index + 2] = b;
    data[index + 3] = a;
}

function radius(itteration) {
    return (1 / Math.pow(1 + 0.5 * itteration, 1)) * initialRadius.value;
}