const invoke = window.__TAURI__.invoke
//const event = window.__TAURI__.event

console.log(window.__TAURI__)


function savePost() {
	const text = document.getElementById("text").value
	const notebook = document.getElementById("notebook").value
	var status_el = document.getElementById("status")
	invoke('save_post', { notebook: notebook, contents: text })
		.then((response) => {
			if (response == true) {
				status_el.textContent = "Saved!"
				status_el.style.color = "green"
				// It's safe to erase the text because posting
				// was confirmed to be successful
				document.getElementById("text").value = ""
			} else {
				status_el.textContent = "Failed to save"
				status_el.style.color = "red"
			}
			status_el.style.display = "inline"
			setTimeout(function () {
				status_el.style.display = "none"
			}, 2500)
		})
}

function openFolder() {
	const notebook = document.getElementById("notebook").value
	invoke('open_folder', { id: notebook })
		.then((response) => {})
}

function updateNotebooks() {
	var notebook_el = document.getElementById("notebook")
	notebook_el.textContent = "" // TODO consider replaceChildren if widely supported
	invoke('notebook_list', {})
		.then((response) => {
			console.log(response)
			for (let i in response) {
				console.log("appending " + response[i])
				var option = document.createElement("option")
				option.value = response[i]
				option.text = response[i]
				notebook_el.appendChild(option)
			}
		})
}

function saveStatus() {
	const text = document.getElementById("text").value
	const notebook = document.getElementById("notebook").value
	invoke('save_state', { text: text, notebook: notebook })
		.then((response) => {})
}

// currently can cause data races when saving to file
function loadState() {
	invoke('load_state', { })
		.then((response) => {
			document.getElementById("text").value = response.text
			document.getElementById("notebook").value = response.notebook

			var text_el = document.getElementById("text")
			text_el.setSelectionRange(text_el.value.length, text_el.value.length);
		})
}

function keydown(e) {
	if (!e) {
		e = event
	}
	if (e.ctrlKey && e.keyCode == 81) {
		window.close()
	}
}

document.getElementById("save_button").addEventListener("click", savePost)
document.getElementById("open_folder").addEventListener("click", openFolder)

document.getElementById("text").addEventListener("keyup", saveStatus)
document.getElementById("notebook").addEventListener("change", saveStatus)

updateNotebooks()
loadState()

document.onkeydown = keydown
