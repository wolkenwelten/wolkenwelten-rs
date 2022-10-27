console.log("Test Loaded!");

let seconds = 0;

setInterval(() => {
    seconds++;
    const s = seconds >= 1 ? "s" : "";
    const msg = seconds+" second" + s + " have passed in V8";
    console.log(msg);
}, 1000)