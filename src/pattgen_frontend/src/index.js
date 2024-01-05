function sendKeywords(keywords) {
    console.log("Sending keywords:", keywords);
    // Implement AJAX call or similar to send data to backend
    // Navigate to the next page after sending data
}

function navigate(direction) {
    console.log("Navigating", direction);
    // Implement logic to navigate to the previous or next page
}

import { pattgen_backend } from "../../declarations/pattgen_backend";

async function testFunction() {
   // here async code
    console.log("Test function called");

    // https://httpbin.org/get

    const res = await pattgen_backend.test();

    console.log(res);
}

document.getElementById('testButton').addEventListener('click', testFunction);