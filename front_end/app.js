const datetime = () => {
    var currentdate = new Date(); 
    var datetime = currentdate.getUTCFullYear() + "-" 
                + (currentdate.getUTCMonth()+1)  + "-"   
                + currentdate.getUTCDate() + "T"
                + currentdate.getUTCHours() + ":"  
                + currentdate.getUTCMinutes() + ":" 
                + currentdate.getUTCSeconds() + "."
                + currentdate.getUTCMilliseconds() + "00Z";
    return datetime;
    }

const uuidv4 = () => {
    return ([1e7]+-1e3+-4e3+-8e3+-1e11).replace(/[018]/g, c =>
        (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
    );
}

const vue = Vue.createApp({
    data() {
        return {
            criminals: [],
            cellPositions: [],
            logs: [],
            admin: false,
            index: 0,
            loginError: "",
            sessionId: null,
            socket: null,
            ip: ""
        }
    },
    async created() {

        this.socket = new WebSocket("wss://localhost:8080/ws/");

        this.socket.onopen = (e) => {
            console.log("Connected to server");
            this.socket.send("")
        };

        this.socket.onmessage = (msg) => {
            console.log(msg)
            this.cellPositions = JSON.parse(msg.data);  
            this.getCells();
        };
          
        this.socket.onclose = (event) => {
            if (event.wasClean) {
              alert(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
            } else {
              // e.g. server process killed or network down
              // event.code is usually 1006 in this case
              alert('[close] Connection died');
            }
        };
          
        this.socket.onerror = function(error) {
            alert(`[error]`);
        };

        this.getAllCriminals()

    },
    methods: {
        login: async function(e) {
            const name = document.querySelector("#loginNameField").value;
            const password = document.querySelector("#loginPasswordField").value;

            const request = {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    name: name,
                    password: password,
                })
            }

            await fetch("https://localhost:8080/login", request)
            .then(response => response.json())
            .then(data => {
                if (data) {
                    this.admin = true;
                    $('#loginModal').modal('hide')
                }
                else {
                    e.preventDefault()
                    this.loginError = "Please recheck your credentials"
                }
            })
        },
        logout: async function() {
            this.admin = false
        },
        addCriminal: async function(e) { // add criminals via values
            e.preventDefault();
            var name_value = document.querySelector("#newCriminalName").value
            var crime_value = document.querySelector("#newCriminalCrime").value
            var dob_value = document.querySelector("#newCriminalDob").value
            var desc_value = document.querySelector("#newCriminalInfo").value
            
            const request = {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    id: uuidv4(),
                    created_at: datetime(),
                    name: name_value,
                    crime: crime_value,
                    dob: dob_value,
                    extras: desc_value,
                    image_link: "placeholder-300x300.webp",
                })
            }
            
            await fetch("https://localhost:8080/criminals/add", request)
            .then(() => {
                this.getAllCriminals();
            })
        },
        getById: async function(event) { // get criminal by id
            let name = event.target.parentElement.firstChild.textContent
            let criminal_id = this.criminals.findIndex((element, index) => element.name == name)
            
            this.index = criminal_id
            document.querySelector("#detCrimName").textContent = this.criminals[criminal_id].name
            document.querySelector("#detCrimCrime").textContent = "Crimes commited: " + this.criminals[criminal_id].crime
            document.querySelector("#detCrimDob").textContent = " |  " + this.criminals[criminal_id].dob
            document.querySelector("#detCrimDesc").textContent = this.criminals[criminal_id].extras
            document.querySelector("#detCrimId").textContent = criminal_id
        },
        getAllCriminals: async function() {
            const requestOptions = {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                }
            };

            await fetch("https://localhost:8080/criminals", requestOptions)
            .then(response => response.json())
            .then(data => {
                console.log(data);
                this.criminals = data;
            })
        },
        edit: async function() { // initiali edit criminal and expand modal
            document.querySelector("#criminalNameUpdInput").value = this.criminals[this.index].name
            document.querySelector("#crimeUpdInput").value = this.criminals[this.index].crime
            document.querySelector("#dobUpdInput").value = this.criminals[this.index].dob
            document.querySelector("#descriptionUpdInput").value = this.criminals[this.index].extras
        },
        finalizeEdit: async function() { // finalize the edit and send data to server
            let criminal = this.criminals[this.index]
            var crimName = document.querySelector("#criminalNameUpdInput").value
            var crimCrime = document.querySelector("#crimeUpdInput").value
            var crimDob = document.querySelector("#dobUpdInput").value
            var crimDesc = document.querySelector("#descriptionUpdInput").value

            const request_options = {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    id: criminal.id,
                    created_at: criminal.created_at,
                    name: crimName,
                    crime: crimCrime,
                    dob: crimDob,
                    extras: crimDesc,
                    image_link: criminal.image_link
                })
            }

            await fetch("https://localhost:8080/criminals/update", request_options)
            .then(() => this.getAllCriminals())
        },
        deleteCriminal: async function(id) { // delete a criminal via it's id
            console.log(this.index)
            const request_options = {
                method: "DELETE",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: this.index
                }
            await fetch("https://localhost:8080/criminals/delete", request_options)
            .then(response => {
                this.getAllCriminals()
            })
        },
        getCells: async function() {
            const group = document.querySelector('#CellGroup')
            const arr = Array.from(group.children);

            console.log(group);

            // find index where a cell may reside
            arr.forEach((element, index) => {
                if (this.cellPositions.find(cell => index == cell.cell_id-1))
                    element.firstChild.classList.add("selectedHolder");
                else
                    element.firstChild.classList.remove("selectedHolder");
            })
        },
        cellSelected: async function(e) {
            let button_id = parseInt(e.target.id)
            let data = JSON.stringify({prisoner_id: this.index, cell_id: button_id})
            this.socket.send(data)
        },
        getLogs: async function(e) {
            console.log("get logs");

            await fetch("https://localhost:8080/logs")
            .then(response => response.json())
            .then(data => {
                this.logs = data;
            })
        }
}
}).mount('#app')