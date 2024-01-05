// function sendKeywords(keywords) {
//     console.log("Sending keywords:", keywords);
//     // Implement AJAX call or similar to send data to backend
//     // Navigate to the next page after sending data
// }

// function navigate(direction) {
//     console.log("Navigating", direction);
//     // Implement logic to navigate to the previous or next page
// }

import { pattgen_backend } from "../../declarations/pattgen_backend";

async function generate() {
    const res = await pattgen_backend.start_gen("shitty flute heavy metal");

    // Check if the response contains 'API_ERR'
    if (res.includes('API_ERR')) {
        console.error("Error in API response:", res);
        return;
    }

    try {
        // Parse the JSON response
        const jsonRes = JSON.parse(res);

        // Extract the 'id' field
        const id = jsonRes.id;
        console.log("Extracted ID:", id);

        const checkStatus = async () => {
            const checkRes = await pattgen_backend.check_gen(id);

            // Check for API_ERR in the check_gen response
            if (checkRes.includes('API_ERR')) {
                console.error("Error in check_gen API response:", checkRes);
                return;
            }

            // Parse the check_gen JSON response
            const checkJsonRes = JSON.parse(checkRes);

            // Check if the generation process is complete and successful
            if (checkJsonRes.status === "succeeded") {
                const outputFile = checkJsonRes.output;
                console.log("Generation succeeded. Output WAV file URL:", outputFile);
                clearInterval(intervalId); // Stop checking
            } else {
                console.log("Generation is still in progress. Current status:", checkJsonRes.status);
            }
        };

        // Start checking every 5 second
        const intervalId = setInterval(checkStatus, 5000);

    } catch (e) {
        console.error("Error processing the response:", e);
    }
}

document.getElementById('testButton').addEventListener('click', generate);