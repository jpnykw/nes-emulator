const input = [...document.querySelectorAll('.input')];
const output = document.querySelector('.output');

document.querySelector('button').addEventListener('click', () => {
    const expect = input[0].value.split('\n');
    const detect = input[1].value.split('\n');

    const result = expect.map((x, index) => {
        const y = detect[index];
        return `${x}, ${y}, ${x == y ? 'OK' : 'NO'}`;
    }).join('\n');

    output.value = result;
});
