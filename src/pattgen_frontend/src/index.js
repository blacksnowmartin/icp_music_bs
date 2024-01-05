import { pattgen_backend } from "../../declarations/pattgen_backend";

async function setPlayerBackground(url) {
    // no need
    // const base64 = await pattgen_backend.fetch_image_as_base64(url);

    const player = document.getElementById('player');
    player.style.backgroundImage = `url(${url})`;
    player.style.backgroundSize = '100% 100%';
    player.style.animation = 'none';
    player.textContent = '';
}

async function createAudioPlayer(audioUrl) {
    // Create an audio element
    const audioPlayer = document.createElement('audio');
    audioPlayer.controls = true; // Add controls like play, pause
    audioPlayer.style.display = 'block'; // Ensure it's visible
    audioPlayer.src = audioUrl;

    // Append the audio player to the div with id 'player'
    const playerDiv = document.getElementById('player');
    playerDiv.appendChild(audioPlayer);
}

async function generate_all() {
    // set testButton disabled
    document.getElementById('testButton').disabled = true;


    const player = document.getElementById('player');
    player.style.animation = 'gradientBG 5s ease infinite';
    player.style.backgroundSize = '400% 400%';

    try {
        const promises = [
            generate_audio("slow arctic death metal funky", (txt) => {
                document.getElementById('player_txt').textContent = txt;
            }),
            generate_image("slow arctic death metal funky")
        ];

        const [audio_url, image_url] = await Promise.all(promises);

        console.log("audio_url:", audio_url);
        console.log("image_url:", image_url);

        const promisesEnd = [
            setPlayerBackground(image_url),
            createAudioPlayer(audio_url)
        ];

        await Promise.all(promisesEnd);

        document.getElementById('testButton').disabled = false;
    } catch (e) {
        document.getElementById('testButton').disabled = false;
        console.error("Error generating content:", e);
    }
}


async function generate_audio(prompt, progressCb) {
    return new Promise(async (resolve, reject) => {
        //
        progressCb("Starting generation ...");
        const res = await pattgen_backend.audio_start_gen(prompt);


        if (res.includes('API_ERR')) {
            console.error("Error in API response:", res);
            progressCb("Error :(");
            reject("Error in API response");
            return;
        }

        try {
            const jsonRes = JSON.parse(res);
            const id = jsonRes.id;
            console.log("Process ID:", id);
            progressCb("In Queue ...");

            let checking = false;
            let intervalId;

            const checkStatus = async () => {
                if (checking) {
                    return;
                }

                checking = true;
                const checkRes = await pattgen_backend.audio_check_gen(id);
                checking = false;

                if (checkRes.includes('API_ERR')) {
                    console.error("Error in check_gen API response:", checkRes);
                    clearInterval(intervalId);
                    progressCb("Error :(");
                    reject("Error in check_gen API response");
                    return;
                }

                const checkJsonRes = JSON.parse(checkRes);

                if (checkJsonRes.status === "succeeded") {
                    const outputFile = checkJsonRes.output;
                    console.log("Generation succeeded. Output WAV file URL:", outputFile);
                    clearInterval(intervalId);
                    progressCb("Post-processing ...");
                    resolve(outputFile);  // Resolving the promise with the outputFile URL
                } else {
                    progressCb("Processing ...");
                    console.log("Audio Generation is still in progress. Current status:", checkJsonRes.status);
                }
            };

            intervalId = setInterval(checkStatus, 1000);

        } catch (e) {
            console.error("Error processing the response:", e);
            progressCb("Error :(");
            reject(e.message);
        }
    });
}

async function generate_image(prompt) {
    return new Promise(async (resolve, reject) => {
        const res = await pattgen_backend.image_start_gen(prompt);

        if (res.includes('API_ERR')) {
            console.error("Error in API response:", res);
            reject("Error in API response");
            return;
        }

        console.log("IMG Response:", res);

        try {
            // Parse the JSON response
            const jsonRes = JSON.parse(res);
            const getURL = jsonRes.urls.get;
            console.log("Get URL:", getURL);

            let checking = false;
            let intervalId;

            const checkStatus = async () => {
                if (checking) {
                    return;
                }

                checking = true;
                const checkRes = await pattgen_backend.image_check_gen(getURL);
                checking = false;

                if (checkRes.includes('API_ERR')) {
                    console.error("Error in check_gen API response:", checkRes);
                    clearInterval(intervalId);
                    reject("Error in check_gen API response");
                    return;
                }

                const checkJsonRes = JSON.parse(checkRes);

                if (checkJsonRes.status === "succeeded") {
                    const outputFile = checkJsonRes.output[0];
                    console.log("Image generation succeeded. Output image URL:", outputFile);
                    clearInterval(intervalId);
                    resolve(outputFile);  // Resolving the promise with the outputFile URL
                } else {
                    console.log("Image Generation is still in progress. Current status:", checkJsonRes.status);
                }
            };

            intervalId = setInterval(checkStatus, 1000);

        } catch (e) {
            console.error("Error parsing JSON response:", e);
            reject("Error parsing JSON response");
        }
    });
}

document.getElementById('testButton').addEventListener('click', generate_all);