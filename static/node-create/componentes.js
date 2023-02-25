function findMatches(regex, str, matches = []) {
    const res = regex.exec(str)
    res && matches.push(res) && findMatches(regex, str, matches)
    return matches
}




const TYPE_MESSAGE = {
    NULL: 0,
    TEXT: 1,
    NUMBER: 2,
    ML: 3
}

class NodeUI {

    static REGEX_GET_TEXT = /([{])([>])\w+([<])([}])/g
    static REGEX_GET_T = /\w+/g


    constructor(key, data) {

        this.id = Number
        this.ar = String
        this.op = []
        this.g = {ent: Number, sai: Number};
        this.val = []
        this.E = {1: {ar: []}, 2: {ar: []}}
        this.term = false
        this.doc = false
        this.ty = Number
        this.typ = String


    }

    static instancie(editor, id) {

        var fevy = true
        editor.on("nodes", (e) => {
            if (e.e.target.offsetParent) {
                if (e.e.target.offsetParent.id === "node-" + id) {
                    if (e.e.target.nodeName === "BUTTON") {
                        e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                        fevy = !fevy
                    }
                }
            }


        })

        const v = new NodeUI(1, {})
        v.id = id
        const js = editor.drawflow.drawflow.Home.data[id]


        v.ty = js.html === "nodeOption" ? TYPE_MESSAGE.NUMBER : TYPE_MESSAGE.TEXT
        for (var i = 1; i <= Object.keys(js.outputs).length; i++) {
            v.op.push(i)

        }


        return v
    }

    build(ex) {
        const js = ex.drawflow.Home.data[this.id]
        if (typeof js.id !== "undefined") {
            if (js.id === this.id) {
                js.data.doc = js.html === "nodeFile"
                js.data.op = this.op
                js.data.type = this.ty
                js.data.term = this.term
                js.data.typ = this.typ

                if (!js.data.hasOwnProperty("g")) {
                    js.data.g = 0
                }
                if (js.data.hasOwnProperty("a")) {
                    js.data.val = js.data.a.match(NodeUI.REGEX_GET_TEXT)


                    if (js.data.val === null) {
                        js.data.val = []
                    }
                } else {
                    js.data.a = ""
                    js.data.val = []
                }


            }
        }


    }
}

class NodeText extends NodeUI {


    constructor(editor, key) {
        super(1, {});


        const id = editor.addNode('github', key, 1, 500, 50, 'github', {"status": "1"}, "NodeText", true);
        this.id = id
        this.typ = "text"
        this.ty = TYPE_MESSAGE.TEXT
        var fevy = true
        editor.on("nodes", (e) => {
            if (e.e.target.offsetParent) {
                if (e.e.target.offsetParent.id === "node-" + id) {
                    if (e.e.target.nodeName === "BUTTON") {
                        e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                        fevy = !fevy
                    }
                }
            }
        })
    }


}
class NodeOption extends NodeUI {

    constructor(editor, key, data, entrada, saida) {
        super(key, data);

        for (var i = 1; i <= saida; i++) {
            this.op.push(i)
        }

        const id = editor.addNode('github', entrada, saida, 500, 50, 'github', {"status": "1"}, "nodeOption"+saida, true);
        this.id = id
        this.typ = "quick_reply"
        this.ty = TYPE_MESSAGE.NUMBER
        var fevy = true
        editor.on("nodes", (e) => {
            if (e.e.target.offsetParent) {
                if (e.e.target.offsetParent.id === "node-" + id) {
                    if (e.e.target.nodeName === "BUTTON") {
                        e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                        fevy = !fevy
                    }
                }
            }

        })
    }

}
class NodeList extends NodeUI {

    constructor(editor, key, data, entrada, saida) {
        super(key, data);

        for (var i = 1; i <= saida; i++) {
            this.op.push(i)
        }

        const id = editor.addNode('github', entrada, saida, 500, 50, 'github', {"status": "1"}, "nodelista"+saida, true);
        this.id = id
        this.typ = "list"
        this.ty = TYPE_MESSAGE.NUMBER
        var fevy = true
        editor.on("nodes", (e) => {
            if (e.e.target.offsetParent) {
                if (e.e.target.offsetParent.id === "node-" + id) {
                    if (e.e.target.nodeName === "BUTTON") {
                        e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                        fevy = !fevy
                    }
                }
            }

        })
    }

}
class NodeML extends NodeUI {


    constructor(editor, key, data) {
        super(key, data);

        for (var i = 1; i <= 3; i++) {
            this.op.push(i)
        }

        const id = editor.addNode('github', 1, 3, 500, 50, 'github', {"status": "1"}, "nodeML", true);
        this.id = id
        this.ty = TYPE_MESSAGE.ML
        var fevy = true
        editor.on("nodes", (e) => {
            if (e.e.target.offsetParent) {
                if (e.e.target.offsetParent.id === "node-" + id) {
                    if (e.e.target.nodeName === "BUTTON") {
                        e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                        fevy = !fevy
                    }
                }
            }

        })
    }

}
class NodeFile extends NodeUI {


    constructor(editor) {
        super(1, {});

        const id = editor.addNode('github', 1, 0, 500, 50, 'github', {"status": "1"}, "nodeFile", true);
        this.id = id
        this.term = true
        var fevy = true
        editor.on("nodes", (e) => {
            if (e.e.target.offsetParent) {
                if (e.e.target.offsetParent.id === "node-" + id) {
                    if (e.e.target.nodeName === "BUTTON") {
                        e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                        fevy = !fevy
                    }
                }
            }

        })
    }


}
class NodeRedirect extends NodeUI {


    constructor(editor) {
        super(1, null);

        const id = editor.addNode('github', 1, 0, 500, 50, 'github', {"status": "1"}, "nodeRedirect", true);
        this.id = id
        this.ty = TYPE_MESSAGE.TEXT
        var fevy = true
        editor.on("nodes", (e) => {


            if (e.e.target.offsetParent) {
                if (e.e.target.offsetParent.id === "node-" + id) {
                    if (e.e.target.nodeName === "BUTTON") {
                        e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                        fevy = !fevy
                    }
                }
            }

        })
    }


}


/**
 *
 * @param {Drawflow} editor
 * @param key
 * @param en
 * @param op
 * @param color
 */

function nodeOption(editor, key) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = " Multipla opcões"
    labe2.style.color = "#000000"

    const selectElement = document.createElement("select")
    selectElement.setAttribute("df-select","")
    selectElement.add(new Option("Texto","text"))
    selectElement.add(new Option("Image","image"))
    selectElement.add(new Option("Video","video"))
    selectElement.add(new Option("Documento","document"))
    selectElement.classList.add("browser-default")

    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#1e4f14"
    status.style.fontSize = "25pt"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"


    const x = document.createElement("label")
    x.setAttribute("df-typ", "quick_reply")
    x.innerText = "Botão 1"
    x.style.fontSize = "16pt"
    x.style.color = "#000"
    x.style.marginRight = "16px"
    header.appendChild(x)

    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(status)
    header.appendChild(selectElement)


    // CORPO


    const label = document.createElement("label")
    label.textContent = "Digite sua pergunta"
    root.appendChild(row(label))


    const input = document.createElement("textarea")
    input.classList.add("materialize-textarea")
    input.style.fontSize = "20pt"
    input.setAttribute("df-a", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput))



        inputnode("Variáveis disponível:", root, true, "#379b65");




    editor.registerNode("nodeOption1", ul);

}
function nodeOption2(editor, key) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = " Multipla opcões"
    labe2.style.color = "#000000"

    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#1e4f14"
    status.style.fontSize = "25pt"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"


    const x = document.createElement("label")
    x.setAttribute("df-typ", "quick_reply")
    x.innerText = "Botão 2"
    x.style.fontSize = "16pt"
    x.style.color = "#000"
    x.style.marginRight = "16px"
    header.appendChild(x)




    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(status)


    const selectElement = document.createElement("select")
    selectElement.setAttribute("df-select","")
    selectElement.add(new Option("Texto","text"))
    selectElement.add(new Option("Image","image"))
    selectElement.add(new Option("Video","video"))
    selectElement.add(new Option("Documento","document"))
    selectElement.classList.add("browser-default")
    header.appendChild(selectElement)

    // CORPO


    const label = document.createElement("label")
    label.textContent = "Digite sua pergunta"
    root.appendChild(row(label))


    const input = document.createElement("textarea")
    input.classList.add("materialize-textarea")
    input.style.fontSize = "20pt"
    input.setAttribute("df-a", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput))



    inputnode("Texto do Botão 1", root, true, "#379b65");
    inputnode("Texto do Botão 2", root, true, "#379b65",2);



    editor.registerNode("nodeOption2", ul,{typ:"quick_reply"});



}
function nodeOption3(editor, key) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = " Multipla opcões"
    labe2.style.color = "#000000"

    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#1e4f14"
    status.style.fontSize = "25pt"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"

    const x = document.createElement("label")
    x.setAttribute("df-typ", "quick_reply")
    x.innerText = "Botão 3"
    x.style.fontSize = "16pt"
    x.style.color = "#000"
    x.style.marginRight = "16px"
    header.appendChild(x)

    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(status)


    const selectElement = document.createElement("select")
    selectElement.setAttribute("df-select","")
    selectElement.add(new Option("Texto","text"))
    selectElement.add(new Option("Image","image"))
    selectElement.add(new Option("Video","video"))
    selectElement.add(new Option("Documento","document"))
    selectElement.classList.add("browser-default")
    header.appendChild(selectElement)
    selectElement.onchange = (ev) => {
        selectElement.setAttribute("df-select",  ev.target.value )
    }

    // CORPO


    const label = document.createElement("label")
    label.textContent = "Digite sua pergunta"
    root.appendChild(row(label))


    const input = document.createElement("textarea")
    input.classList.add("materialize-textarea")
    input.style.fontSize = "20pt"
    input.setAttribute("df-a", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput))



    inputnode("Texto do Botão 1", root, true, "#379b65");
    inputnode("Texto do Botão 2", root, true, "#379b65",2);
    inputnode("Texto do Botão 3", root, true, "#379b65",3);


    editor.registerNode("nodeOption3", ul);

}
function nodeText(editor, key) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = " Multipla opcões"
    labe2.style.color = "#000000"

    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#1e4f14"
    status.style.fontSize = "25pt"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"

    const x = document.createElement("label")
    x.innerText = "Textos"
    x.style.fontSize = "16pt"
    x.style.color = "#000"
    x.style.marginRight = "16px"
    header.appendChild(x)



    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(status)


    // CORPO


    const label = document.createElement("p")
    label.textContent = "Texto"
    label.style.color = "#000"
    root.appendChild(row(label))


    const input = document.createElement("textarea")
    input.style.fontSize = "20pt"
    input.classList.add("materialize-textarea")
    input.setAttribute("df-a", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput))


    //inputnode("Variáveis disponível:", root, false, "#379b65")
    //inputnode("dia = Bom dia/Boa Tarde/Boa Noite; {>dia<}", root, false, "#379b65")
    //inputselect("Gatilhos de Requisições", root, true)
    //inputnode("Mensagens de Erro", root, false, "#000000")


    editor.registerNode("NodeText", ul);

}

function nodeLista(editor, key,index) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = "Lista de Opções"
    labe2.style.color = "#000000"


    const container = document.createElement("div")
    container.classList.add("listop")

    for (let e =0 ; e< index ;e++){
        const i = document.createElement("input")
        i.classList.add("statuss")
        i.placeholder = "texto "+ (e+1) ;
        i.value = "edite aqui "  + (e+1)
        i.setAttribute("df-l-"+e, "")
        container.appendChild(row(i))

    }

    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#1e4f14"
    status.style.fontSize = "25pt"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"


    const x = document.createElement("label")
    x.setAttribute("df-typ", "quick_reply")
    x.innerText = "Botão 1"
    x.style.fontSize = "16pt"
    x.style.color = "#000"
    x.style.marginRight = "16px"
    header.appendChild(x)

    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(status)






    // CORPO


    const label = document.createElement("label")
    label.textContent = "Texto"
    root.appendChild(row(label))


    const input = document.createElement("textarea")
    input.classList.add("materialize-textarea")
    input.style.fontSize = "20pt"
    input.setAttribute("df-a", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput))


    root.appendChild(row(container))



   // inputnode("Variáveis disponível:", root, true, "#379b65");




    editor.registerNode("nodelista"+index, ul);

}


async function nodeML(editor, key) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = "ML"
    labe2.style.color = "#000000"

    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#1e4f14"
    status.style.fontSize = "25pt"

    const s = document.createElement("div")
    s.innerHTML = "<h6 style='color: green;margin-right: 5px'><b>Aceitação</b></h6><h6 style='color:blue;margin-right: 5px'><b>Dúvida</b></h6><h6 style='color:red;margin-right: 5px'><b>Negação</b></h6>"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"


    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(s)
    header.appendChild(status)


    // CORPO


    const label = document.createElement("h6")
    label.innerHTML = "<b>Tag de ML</b>"
    label.style.color = "#000000"
    root.appendChild(row(label))

   /* const input = document.createElement("input")
    input.style.fontSize = "20pt"
    input.setAttribute("df-tag", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput)) */

  await  inputselectML("", root, true)


    inputselect("Gatilhos de Requisições", root, true)

    editor.registerNode("nodeML", ul);

}
function nodeRedirect(editor) {


    const labe2 = document.createElement("h5")
    labe2.textContent = "Redirecionamento"
    labe2.style.color = "#000000"

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"
    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#020202"
    status.style.fontSize = "25pt"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"


    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(status)


    // CORPO


    const label = document.createElement("label")
    label.textContent = "Bloco"
    root.appendChild(row(label))
    const input = document.createElement("input")
    input.style.fontSize = "25pt"
    input.setAttribute("df-redirect", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput))
    editor.registerNode("nodeRedirect", ul);

}
function nodeFile(editor) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box")
    root.style.display = "none"
    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")
    const header = document.createElement("div")
    header.classList.add("header", "card-panel")
    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = "Envio de Aquivo"
    labe2.style.color = "#000000"

    const status = document.createElement("label")
    status.setAttribute("df-status", "")
    status.textContent = "1"
    status.classList.add("conut")
    status.style.color = "#1e4f14"
    status.style.fontSize = "25pt"


    const i = document.createElement("input")
    i.classList.add("statuss")
    i.setAttribute("df-status", "")
    header.appendChild(i)
    i.style.display = "none"


    const button = document.createElement("button")
    button.textContent = "Configurações"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)
    header.appendChild(status)


    // CORPO


    const label = document.createElement("label")
    label.textContent = "Digite sua pergunta"
    root.appendChild(row(label))


    const input = document.createElement("textarea")
    input.style.fontSize = "20pt"
    input.classList.add("materialize-textarea")
    input.setAttribute("df-a", "")
    const containerInput = document.createElement("div")
    containerInput.classList.add('input-field', "m12")
    containerInput.appendChild(input)
    root.appendChild(row(containerInput))


    inputnode("Variáveis disponível:", root, false, "#379b65")
    inputnode("dia = Bom dia/Boa Tarde/Boa Noite; {>dia<}", root, false, "#379b65")
    inputselect("Gatilhos de Requisições", root, true)

    editor.registerNode("nodeFile", ul);

}


function inputnode(text, root, isInput, color,index = 1) {
    const label = document.createElement("h6")
    label.innerHTML = "<b>"+text+"</b>"
    label.classList.add("col", "m12")
    label.style.color = "#000000"
    label.style.color = color


    root.appendChild(row(label))

    if (isInput) {

        const input = document.createElement("input")
        input.setAttribute("df-a-"+index, "")
        const containerInput = document.createElement("div")
        containerInput.classList.add('input-field', "m12")
        containerInput.appendChild(input)
        root.appendChild(row(containerInput))
    }
}


function inputerro(text, root) {


    for (var i = 1; i <= 2; i++) {

        const label = document.createElement("label")
        label.textContent = text + " " + i
        label.classList.add("col", "m12")
        root.appendChild(row(label))

        const input = document.createElement("input")
        input.style.fontSize = "20pt"
        input.setAttribute("df-e-" + i, "")
        const containerInput = document.createElement("div")
        containerInput.classList.add('input-field', "m12")
        containerInput.appendChild(input)
        root.appendChild(row(containerInput))
    }
}

function inputselect(text, root, isInput) {


    const label = document.createElement("h6")
    label.innerHTML = "<b>"+text+"</b>"
    label.classList.add("col", "m12")
    label.style.color = "#000000"
    label.style.marginTop = "10px"


    root.appendChild(row(label))

    if (isInput) {

        const input = document.createElement("select")
        input.setAttribute("df-g", "")
        input.classList.add("browser-default")
        input.appendChild(new Option("Nenhum", "", true))
        input.appendChild(new Option("Verifica CPF", "5"))
        const containerInput = document.createElement("div")
        containerInput.classList.add('input-field', "m12")
        containerInput.appendChild(input)
        root.appendChild(row(containerInput))
    }
}




async function inputselectML(text, root, isInput) {

    let fre = await  fetch("/api/ml/all", {method: 'get'});
    let valuesML = await fre.json()

    const label = document.createElement("h6")
    label.innerHTML = "<b>"+text+"</b>"
    label.classList.add("col", "m12")
    label.style.color = "#000000"
    label.style.marginTop = "10px"


    root.appendChild(row(label))

    if (isInput) {

        const input = document.createElement("select")
        input.setAttribute("df-ml", "")
        input.classList.add("browser-default")

        input.appendChild(new Option("Nenhum", "", true))
        Array.from(valuesML).forEach((v, index) => {

            input.appendChild(new Option(v.name, v.id +""))


        })
        const containerInput = document.createElement("div")
        containerInput.classList.add('input-field', "m12")
        containerInput.appendChild(input)
        root.appendChild(row(containerInput))
    }
}


function row(elemt) {
    const e = document.createElement("div")
    e.classList.add("col", "m12")
    e.appendChild(elemt)
    return e;
}

function rootNode(editor, key, en, op, color) {

    const root = document.createElement("div")
    root.classList.add("card-panel", "box", "row")
    root.style.display = "none"


    const ul = document.createElement("ul")
    ul.style.backgroundColor = "tranparent"


    const li = document.createElement("li")
    const li2 = document.createElement("li")


    const header = document.createElement("div")
    header.classList.add("header", "card-panel")


    li.appendChild(header)
    li2.appendChild(root)
    ul.appendChild(li)
    ul.appendChild(li2)


    const labe2 = document.createElement("h5")
    labe2.textContent = "Texto Pergunta " + key
    labe2.style.color = "#000000"

    const button = document.createElement("button")
    button.textContent = "Expandir"
    button.classList.add("btn", "gradi", "no-uppercase")
    header.appendChild(button)
    header.appendChild(labe2)


    // CORPO

    const labelt = document.createElement("label")
    labelt.textContent = "Tipo de entrada ?"
    root.appendChild(labelt)


    var stt = document.createElement("div")
    stt.innerHTML = '<div class="input-field col m12 ">\n' +
        '    <select class="browser-default">\n' +
        '      <option value="0">Numero</option>\n' +
        '      <option value="1">Texto</option>\n' +
        '    </select>\n' +
        '  </div>'

    root.appendChild(stt)

    for (let i = 1; i <= 1; i++) {


        const label = document.createElement("label")

        label.classList.add("conut")
        label.textContent = i + ""
        root.appendChild(row(label))
        label.style.color = color[i - 1]
        const input = document.createElement("textarea")
        input.setAttribute("df-" + key + "-a-" + i, "")
        const containerInput = document.createElement("div")
        containerInput.classList.add('input-field', "m12")
        containerInput.appendChild(input)
        root.appendChild(row(containerInput))

    }

    const labelSelect = document.createElement("label")
    labelSelect.textContent = "Techo final ?"
    root.appendChild(labelSelect)


    var st = document.createElement("div")
    st.innerHTML = '<div class="input-field col s12">\n' +
        '    <select class="browser-default">\n' +
        '      <option value="0">Não</option>\n' +
        '      <option value="1">Sim</option>\n' +
        '    </select>\n' +
        '  </div>'

    root.appendChild(st)


    const labelgati = document.createElement("label")
    labelgati.textContent = "Escolha seu gatilho de requisições?"
    root.appendChild(labelgati)

    var stg = document.createElement("div")
    stg.innerHTML = '<div class="input-field col s12">\n' +
        '    <select class="browser-default">\n' +
        '      <option value="0">Não</option>\n' +
        '      <option value="1">Sim</option>\n' +
        '    </select>\n' +
        '  </div>'

    root.appendChild(stg)
    root.appendChild(document.createElement("br"))


    var varr = document.createElement("div")
    varr.innerHTML = '<div class="input-field col s6">\n' +
        '          <input placeholder="Exemplo: bola,nome "  id="vall" type="text" class="validate">\n' +
        '          <label for="vall">Suas variáveis</label>\n' +
        '        </div>'


    root.appendChild(varr)


    for (let i = 1; i <= 2; i++) {


        const label = document.createElement("label")

        label.classList.add("conut")
        label.textContent = "Erro e tentativa " + i
        root.appendChild(label)
        const input = document.createElement("textarea")
        input.classList.add('materialize-textarea')
        input.placeholder = "Erro " + i
        input.setAttribute("df-" + key + "-e-" + i, "")
        const containerInput = document.createElement("div")
        containerInput.classList.add('input-field')
        containerInput.appendChild(input)
        root.appendChild(containerInput)

    }


    var data = {status: key}
    editor.registerNode(data.status + "", ul);
    const id = editor.addNode('github', en, op, 500, 50, 'github', data, data.status + "", true);
    //  var  temp = editor.container.querySelector("#node-"+id)
    //  var box =  temp.querySelector(".box").style
    //  box.display = "block"

    var fevy = true

    editor.on("nodes", (e) => {

        if (e.e.target.offsetParent.id === "node-" + id) {
            if (e.e.target.nodeName === "BUTTON") {
                e.e.target.offsetParent.querySelector(".box").style.display = fevy ? "block" : "none"
                fevy = !fevy
                // e.e.target.offsetParent.mousemove();


            }
        }
    })


}
