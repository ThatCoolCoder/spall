const randomService = {
    randomFloat(min, max) {
        var delta = max - min;
        return Math.random() * delta + min;
    },
    randomInt(min, max) {
        return Math.floor(this.randomFloat(min, max));
    },
    randomChoice(data) {
        return data[this.randomInt(0, data.length)];
    }
}