const weatherService = {
    WeatherForecast: class {
        constructor(data={}) {
            this.snapshots = data;
        }

        addSnapshot(label, value) {
            this.snapshots[label] = value;
        }
    },
    WeatherSnapshot: class {
        // class representing snapshot of weather at particular moment

        constructor(temperature, rainProbability, condition, humidity) {
            this.temperature = temperature;
            this.rainProbability = rainProbability;
            this.condition = condition;
            this.humidity = humidity;
        }
    },
    WeatherCondition: {
        sunny: "Sunny",
        partlyCloudy: "Partly cloudy",
        cloudy: "Cloudy",
        storm: "Stormy"
    },

    generateForecast(from, distance) {
        // From should be a date, distance is a number saying how many days in advance.

        var forecast = new this.WeatherForecast();
        var crntDate = from;
        for (var i = 0; i < distance; i ++) {
            crntDate.setDate(crntDate.getDate() + 1);
            forecast.addSnapshot(new Date(crntDate.getTime()), this.currentWeather());
        }
        return forecast;
    },

    currentWeather() {
        return new this.WeatherSnapshot(randomService.randomInt(12, 35),
            Math.random(),
            randomService.randomChoice(Object.keys(this.WeatherCondition)),
            Math.random());
    }
}