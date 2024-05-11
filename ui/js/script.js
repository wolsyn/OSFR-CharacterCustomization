const { invoke } = window.__TAURI__.tauri;
document.addEventListener('DOMContentLoaded', (event) => {
    function mapGenderRaceToValues(genderRaceValue) {
        switch (genderRaceValue) {
            case '60':
                return { gender: "f", species: "human" };
            case '1':
                return { gender: "m", species: "human" };
            case '61':
                return { gender: "f", species: "fairy" };
            case '2':
                return { gender: "m", species: "fairy" };
            case '0':
                return { gender: "", species: "" };
        }
    }
    function mapGenderRaceToGender(genderRaceValue) {
        switch (genderRaceValue) {
            case '60':
                return 'f';
            case '1':
                return 'm';
            case '61':
                return 'f';
            case '2':
                return 'm';
            case '0':
                return '';
        }
    }

    const extrasSelect = document.getElementById('extras');

    var genderrace = document.getElementById('genderrace');
    genderrace.addEventListener('change', function () {
        const genderRaceValue = genderRace.value;
        var extras = document.getElementById('Extras');
        if (genderRaceValue === '60') {
            extras.hidden = true;
            alert("There is no available Extras for Humans!");
            return 0;
        }
        extras.hidden = false;

        const { gender, species } = mapGenderRaceToValues(genderRaceValue);

        extrasSelect.innerHTML = '';

        invoke('model_extras', { gender, species }).then((payload) => {
            const extrasArray = payload;

            extrasArray.forEach(extra => {
                var option = document.createElement('option');
                option.value = extra.addr;
                option.text = extra.name;
                extrasSelect.appendChild(option);
            })
        });
    });

    const modelHairType = document.getElementById('hairtype');
    var genderrace = document.getElementById('genderrace');

    genderrace.addEventListener('change', function () {
        modelHairType.innerHTML = '';
        const genderRaceValue = genderRace.value;

        const hairGender = mapGenderRaceToGender(genderRaceValue);
        extrasSelect.innerHTML = '';

        invoke('hair_type', { gender: hairGender }).then((payload) => {
            const hairArray = payload;

            const select = document.getElementById('hairtype');
            hairArray.forEach(hair => {
                const option = document.createElement('option');
                option.value = hair.addr;
                option.text = hair.name;
                select.appendChild(option);
            });
        });

    });


    invoke('hair_color').then((payload) => {
        const paintArray = payload;
        const select = document.getElementById('haircolor');

        paintArray.forEach(haircolor => {
            var option = document.createElement('option');
            option.value = haircolor.color;
            option.text = haircolor.name;
            select.appendChild(option);
        })
    });



    invoke('facepaint').then((payload) => {
        const paintArray = payload;
        const select = document.getElementById('facepaint');

        paintArray.forEach(paint => {
            var option = document.createElement('option');
            option.value = paint.texture_alias;
            option.text = paint.texture_alias;
            select.appendChild(option);
        })
    });

    invoke('eye_color').then((payload) => {
        const select = document.getElementById('eyecolor');

        payload.forEach(paint => {
            var option = document.createElement('option');
            option.value = paint.name;
            option.text = paint.color;
            select.appendChild(option);
        })
    });

});

var operationQueue = [];

async function enqueueOperation(operation) {
    const operationPromise = new Promise(async (resolve, reject) => {
        try {
            await operation();
            resolve();
        } catch (error) {
            reject(error);
        }
    });

    operationQueue.push(operationPromise);

    if (operationQueue.length === 1) {
        await operationQueue[0];
    }

    operationQueue.shift();
}

const createBtn = document.getElementById('createBtn');

const username = document.getElementById('username');
const surname = document.getElementById('surname')
const genderRace = document.getElementById('genderrace');
const hairType = document.getElementById('hairtype');
const hairColor = document.getElementById('haircolor');
const eyeColor = document.getElementById('eyecolor');
const facePaint = document.getElementById('facepaint');
const modelExtras = document.getElementById('extras');
const skintone = document.getElementById('skintone');


createBtn.addEventListener('click', async function () {
    var usernameValue = username.value;
    var surnameValue = surname.value;
    var genderRaceValue = genderRace.value;
    var hairTypeValue = hairType.value;
    var hairColorValue = hairColor.value;
    var eyeColorValue = eyeColor.value;
    var facePaintValue = facePaint.value;
    var modelExtrasValue = modelExtras.value;
    var skintoneValue = skintone.value;


    const createBtn = document.getElementById('createBtn');

    createBtn.disabled = true;

    try {
        await enqueueOperation(() => invoke('new_character', { username: usernameValue, surname: surnameValue }));
        await enqueueOperation(() => invoke('set_genderace', { username: usernameValue, surname: surnameValue, genderrace: Number(genderRaceValue) }));
        await enqueueOperation(() => invoke('set_facepaint', { username: usernameValue, surname: surnameValue, facepaint: facePaintValue }));
        await enqueueOperation(() => invoke('set_skintone', { username: usernameValue, surname: surnameValue, newskintone: skintoneValue }));
        await enqueueOperation(() => invoke('set_hair', { username: usernameValue, surname: surnameValue, hairtype: hairTypeValue, haircolor: Number(hairColorValue) }));
        await enqueueOperation(() => invoke('set_eyes', { username: usernameValue, surname: surnameValue, color: Number(eyeColorValue) }));
        await enqueueOperation(() => invoke('set_extras', { username: usernameValue, surname: surnameValue, extra: modelExtrasValue }));
    } finally {
        createBtn.disabled = false;
        alert('Character created with success!');
    }
});
