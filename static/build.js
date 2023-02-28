var blockSelected = -1;
let construtor = 1;
let nameapp = ""

let perguntamodal = null
let perguntamodalLista = null

window.addEventListener('DOMContentLoaded', async function (e) {
    M.Modal.init(document.querySelector(".modal"));
   perguntamodal =  M.Modal.init(document.querySelector("#perguntamodal"));
    perguntamodalLista =  M.Modal.init(document.querySelector("#perguntamodallista"));

    M.FormSelect.init(document.querySelectorAll('select'));

    document.querySelector('#click-new-whatsapp').addEventListener('click', async (e) => {


        var ape = document.querySelector('#ape').value



        if (ape !== "") {


            let headers = {
                'Content-Type': 'application/json',
                //  'x-access-token': localStorage.getItem('login-token')
            }
          /*  var data = {apelido: ape}
            var fre = await fetch("/api/newbot", {method: 'post', headers: headers, body: JSON.stringify(data)});
            if (fre.status === 201) {

                document.querySelector('#ape').value = ""
                M.Modal.getInstance(document.querySelector('#newwhatsapp-model')).close()
                M.toast({html: 'Novo serviço criado'})
                await values()
            }
            */
            nameapp = ape
            document.querySelector('#ape').value = ""
            blockSelected = 0
            M.Modal.getInstance(document.querySelector('#newwhatsapp-model')).close()

        } else {
            M.toast({html: 'Verifique seus campos varios'})

        }


    })
    document.querySelector('#nt').addEventListener('click', async (e) => {
        if (blockSelected !== -1) {
            nt()
        } else {
            M.toast({html: ' <label class="white-text" style="margin-right: 10px;" > Arvore não selecionado </label>  '})

        }

    })
    document.querySelector('#no').addEventListener('click', async (e) => {
        if (blockSelected !== -1) {
            openModal()
        }

    })
    document.querySelector('#nr').addEventListener('click', async (e) => {
        if (blockSelected !== -1) {
            redi()
        }

    })
    document.querySelector('#nf').addEventListener('click', async (e) => {
        if (blockSelected !== -1) {
            nfile()
        }

    })
    document.querySelector('#ml').addEventListener('click', async (e) => {
        if (blockSelected !== -1) {
            ml()
        }
    })
    document.querySelector("#saveconstruction").addEventListener("click", async (e) => {

        if (blockSelected !== -1) {


            e.target.classList.add("disabled")

            let headers = {
                'Content-Type': 'application/json',
            }
            var data = { app:nameapp.trim(), value: export2()}
            var fre = await fetch("/whatsapp/bot/insert", {
                method: 'post',
                headers: headers,
                body: JSON.stringify(data)
            });



            if (fre.status === 200) {
                e.target.classList.remove("disabled")
                M.toast({"html": "Salvo com sucesso"})

            } else {
                e.target.classList.remove("disabled")
               // M.toast({"html": "Operação não concluída,tente novamente"})


            }
        }

    })
    document.querySelector("#deleteconstruction").addEventListener("click", async (e) => {

        if (blockSelected !== -1) {

            if (confirm("Desejar realmente apagar ?")) {

                let headers = {
                    'Content-Type': 'application/json',
                    'x-access-token': localStorage.getItem('login-token')
                }
                var data = {id: blockSelected, idc: localStorage.getItem("idclient")}
                var fre = await fetch("/back/api/whats/build/delete", {
                    method: 'post',
                    headers: headers,
                    body: JSON.stringify(data)
                });


                if (fre.status === 200) {
                    editor.clear()
                    blockSelected = -1
                    document.querySelector("#deleteconstruction").classList.add("disabled")
                    document.querySelector('#blockselect').innerHTML = ''
                    await values()

                }
            }


        }

    })


    await values()


})



async function values() {


    var fre = await fetch("/whatsapp/bot/get", {method: 'get'});
    var values = await fre.json()


    var container = document.querySelector('#myservicees')
    container.innerHTML = ''


    for (var v = 0; v < values.length; v++) {
        await blocos(container, Array.from(values)[v])
    }

}

async function load(value) {

    try {
        editor.clear()
        if (values !== null) {
            importaConversa(value)
        }

    } catch (e) {
        console.log(e)
    }
}

async function blocos(container, value) {


    var card = document.createElement('li');

    var block = '<div class="center-align waves-effect" style="margin: 5px;border-radius: 20px ;padding-left: 15px;padding-right 15px;width:225px;margin-right:10px;background-color:#F7F9F9;"> ' +
        ' <label class="black-text"><b>' + value.app + '</b></label>' +
        '<a class="btn z-depth-0 transparent"><i class="material-icons btnani">launch  </i></a> </div>'

    card.innerHTML = block
    container.appendChild(card)

    card.addEventListener('click', async (e) => {


        if (blockSelected !== -1) {

            if (confirm("Desejar realmente sair")) {
                document.querySelector('#blockselect').innerHTML = 'Conversa:<b>' + value.app + '</b>'
                nodesContrution = []
                blockSelected = value.id
                await load(value.value)
                nameapp = value.app
            }
        } else {
            document.querySelector('#blockselect').innerHTML = 'Conversa:<b>' + value.app + '</b>'
            nodesContrution = []
            blockSelected = value.id
            await load(value.value)
            nameapp = value.app
            document.querySelector("#deleteconstruction").classList.remove("disabled")
        }


    })


}


function ml() {


    perguntamodalLista.open();
  /*  var key = 0

    if (nodesContrution.length > 0) {
        key = 1
    }
    nodesContrution.push(new NodeList(editor, key, {},key,1))  */
}

function nt() {


    var key = 0

    if (nodesContrution.length > 0) {
        key = 1
    }
    nodesContrution.push(new NodeText(editor, key, {}))
}

function nfile() {

    if (nodesContrution.length >= 1) {
        nodesContrution.push(new NodeFile(editor))
        M.toast({"html": "Bloco de envio criado"})

    } else {
        M.toast({"html": "Deve ter no mínimo 1 bloco de conversa"})
    }
}

function redi() {


    if (nodesContrution.length >= 3) {
        nodesContrution.push(new NodeRedirect(editor))
    } else {
        M.toast({"html": "Deve ter no mínimo 3 blocos de conversas"})
    }

}




function openModal() {
  //  const response = M.Modal.getInstance(document.getElementById('perguntamodal'));

   // console.log(response)
    perguntamodal.open();

}


function closeModal() {
    const response = M.Modal.getInstance(document.querySelector('#perguntamodal'));
    response.close();
}

function newPerguntaOp() {

    if(document.getElementById("numop").value !== ""){


        var key = 0
        if (nodesContrution.length > 0) {
            key = 1
        }


        const op = parseInt(document.getElementById("numop").value)
        nodesContrution.push(new NodeOption(editor, 1, {"status": "1","a-1":"+ Informação","a-2":"Contratar","a-3":"Voltar"}, key, op))
        document.getElementById("numop").value = ""
        closeModal();

    }



}
function newPerguntaOpLista() {

    if(document.getElementById("numopl").value !== ""){


        var key = 0
        if (nodesContrution.length > 0) {
            key = 1
        }
        const op = parseInt(document.getElementById("numopl").value)
        nodesContrution.push(new NodeList(editor, 1, {}, key, op))
        document.getElementById("numopl").value = ""
        perguntamodalLista.close();

    }



}


/**
 *
 * @param {HTMLInputElement} e
 */
const chargeNumber = (e) => {

    console.log(e)
       if(e.value === "1" || e.value === "2" || e.value === "3"){

       }else {
           e.value = ""
       }
}

