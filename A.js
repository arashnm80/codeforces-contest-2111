const readline = require("readline");
 
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});
 
let inputLines = [];
 
rl.on("line", (line) => {
    inputLines.push(line.trim());
}).on("close", () => {
    const t = parseInt(inputLines[0]);
    let currentLine = 1;
 
    function func(nums, x) {
        nums.sort((a, b) => a - b);
        nums[0] = Math.min(2 * nums[1] + 1, x);
        return nums;
    }
 
    for (let i = 0; i < t; i++) {
        const x = parseInt(inputLines[currentLine++]);
        let nums = [0, 0, 0];
        let counter = 0;
 
        while (!(nums[0] === x && nums[1] === x && nums[2] === x)) {
            nums = func(nums, x);
            counter++;
        }
 
        console.log(counter);
    }
});
