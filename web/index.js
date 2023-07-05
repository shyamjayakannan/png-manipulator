import * as wasm from "../pkg/png_manipulator";

const chunk_type = "ruSt";

const main = document.querySelector(".main");
const inputs = document.querySelectorAll(".input");
const droparea = document.querySelectorAll(".drop");
const p = document.querySelector(".drop p");
const removes = document.querySelectorAll(".remove");
const uploadImage = document.getElementById("uploadImage");
const encode_button = document.getElementById("encode");
const decode_button = document.getElementById("decode");
const remove_button = document.getElementById("remove");
const texts = document.querySelectorAll(".text");
const browse_button = document.getElementById("browse");

let file = null;
let chunk_exists = false;
let message = null;

droparea.forEach(area => {
    area.addEventListener("dragover", e => {
        e.preventDefault();

        area.classList.add("drop_over");
    });

    area.addEventListener("dragleave", e => {
        e.preventDefault();

        area.classList.remove("drop_over");
    });

    area.addEventListener("drop", e => {
        e.preventDefault();

        area.classList.remove("drop_over");

        if (e.dataTransfer.files[0].type.endsWith("png") && !file) {
            removes.forEach(element => element.classList.remove("invisible"));
            texts.forEach(element => element.classList.remove("invisible"));
            p.style.color = "white";
            browse_button.classList.add("invisible");
            addFile(e.dataTransfer.files[0]);
        }
    });
});

inputs.forEach(input => {
    input.addEventListener("change", e => {
        if (e.target.files[0].type.endsWith("png") && !file) {
            removes.forEach(element => element.classList.remove("invisible"));
            texts.forEach(element => element.classList.remove("invisible"));
            p.style.color = "white";
            browse_button.classList.add("invisible");
            
            addFile(e.target.files[0]);
        }

        setTimeout(() => scrollBy({ top: 100, behavior: "smooth" }), 800);
    });
});

removes.forEach(remove => {
    remove.addEventListener("click", e => {
        uploadImage.style.background = "transparent";
        texts.forEach(element => element.classList.add("invisible"));
        removes.forEach(element => element.classList.add("invisible"));
        p.style.color = "gray";
        
        inputs.forEach(input => input.value = "");
        file = null;
        
        const a = document.querySelectorAll("a");
        
        if (a) a.forEach(link => link.remove());
        
        decode_button.classList.add("invisible");
        remove_button.classList.add("invisible");
        encode_button.classList.add("invisible");
        browse_button.classList.remove("invisible");
    });
});

texts.forEach(text => {
    text.addEventListener("input", e => {
        message = e.target.value;

        if (message && !chunk_exists) {
            encode_button.classList.remove("invisible");
        }
        else {
            encode_button.classList.add("invisible");
        }
    })
});

encode_button.addEventListener("click", e => {
    try {
        if (document.querySelector(".ENCODED")) return;

        const fileDownload = new Blob([wasm.encode(file, chunk_type, message)], { type: "image/png" });

        let a = document.createElement("a");
        a.href = URL.createObjectURL(fileDownload);
        a.classList.add("ENCODED");
        a.setAttribute("download", "");
        a.classList.add("label");
        a.innerHTML = "DOWNLOAD ENCODED FILE";

        main.appendChild(a);
    } catch (error) {
        alert(error);
    }

    scrollTo({ top: 100, behavior: "smooth" });
});

remove_button.addEventListener("click", e => {
    try {
        if (document.querySelector(".REMOVED")) return;

        const fileDownload = new Blob([wasm.remove(file, chunk_type)], { type: "image/png" });
        texts[0].value = "Message removed";

        let a = document.createElement("a");
        a.href = URL.createObjectURL(fileDownload);
        a.classList.add("REMOVED");
        a.setAttribute("download", "");
        a.classList.add("label");
        a.innerHTML = "DOWNLOAD REMOVED FILE";

        main.appendChild(a);

        const reader = new FileReader();

        reader.readAsArrayBuffer(fileDownload);
        reader.onload = () => {
            file = reader.result;
            checkForChunk(file, chunk_type);
        }
    } catch (error) {
        alert(error);
    }

    scrollTo({ top: 100, behavior: "smooth" });
});

decode_button.addEventListener("click", e => {
    try {
        texts[0].value = `Message: ${wasm.decode(file, chunk_type)}`;
    } catch (error) {
        alert(error);
    }
});

function addFile(image) {
    const reader = new FileReader();

    file = image;

    reader.readAsDataURL(file);
    reader.onload = () => {
        uploadImage.style.backgroundImage = `url("${reader.result}")`;
        uploadImage.style.backgroundSize = "contain";
        uploadImage.style.backgroundRepeat = "no-repeat";
    }

    setTimeout(() => {
        reader.readAsArrayBuffer(file);
        reader.onload = () => {
            try {
                file = reader.result;
                checkForChunk(file, chunk_type);
            } catch (error) {
                alert(error);
            }
        }
    }, 500);
}

function checkForChunk(file, chunk_type) {
    texts[0].value = "";

    chunk_exists = wasm.check_for_chunk(file, chunk_type);

    if (chunk_exists) {
        decode_button.classList.remove("invisible");
        remove_button.classList.remove("invisible");
        texts.forEach(text => {
            text.setAttribute('disabled', 'true');
            text.setAttribute('placeholder', 'Remove the current message to encode a new one');
        });
    } else {
        decode_button.classList.add("invisible");
        remove_button.classList.add("invisible");
        texts.forEach(text => {
            text.setAttribute('placeholder', 'Message to encode');
            text.removeAttribute('disabled');
        });
    }
}