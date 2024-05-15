<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";


const searchQuery = ref();
const menuVisible = ref(false);
const cityName = ref("");
const countryName = ref("");
const currentWeather = ref({
  date: ["", "", "", "", ""],
  temp: 0.0,
  feelslike: 0.0,
  pressure: 0,
  humidity: 0,
  visibility: 0,
  sunrise: "HH:MM",
  sunset: "HH:MM"
});
const cityList = ref([{ name: "", state: "", lat: "", lob: "", country: "" }])

const hourlyForecast = ref({ forecast: [{ time: "HH:MM", icon: "", temp: 0.0 }] });
const fiveDayForecast = ref({ forecast: [{ date: ["", "", "", "", ""], icon: "", temp: 0.0 }] });
const airPollution = ref({ pm25: "", so2: "", no2: "", o3: "", status: ""})


async function searchCity() {
  menuVisible.value = true
  const city = searchQuery.value;

  const json = await invoke('search_city', { name: city })
    .then((data) => JSON.parse(data.replaceAll("'", '"')).result);

  cityList.value = json
}


async function parseCity(city, event) {
  menuVisible.value = false;
  searchQuery.value = "";

  if (city.country == "UA") {
    countryName.value = "Україна";
  } else {
    countryName.value = city.country;
  }

  cityName.value = city.name;

  parseWeather(city.lat, city.lon);
  getAirIndex(city.lat, city.lon);
}


async function parseWeather(lat, lon) {
  const json = await invoke('get_weather', { lat, lon })
    .then((data) => JSON.parse(data.replaceAll("'", '"')));

  currentWeather.value = json.now;
  hourlyForecast.value = json.hourlyforecast;
  fiveDayForecast.value = json.fivedayforecast;
}


async function getAirIndex(lat, lon) {
  const json = await invoke('get_airindex', { lat, lon })
    .then((data) => JSON.parse(data.replaceAll("'", '"')));

  airPollution.value = json;
}

onMounted(() => {
  invoke('search_city', { name: "Львів" })
    .then((data) => {
      const js = JSON.parse(data.replaceAll("'", '"')).result
      parseCity(js[0])
    });
});
</script>


<template>
  <div class="container">
    <div class="topbar">
      <span class="app-name">JustWeather</span>
      <div class="search-bar">
        <input class="search-input" v-model="searchQuery" v-on:keyup.enter="(event) => searchCity(city, event)" placeholder="Шукати місто..."
          type="text" />
        <img src="./assets/icons/search.svg" class="search-icon" />
      </div>
    </div>

    <div class="search-menu" :class="{active_search_menu: menuVisible}">
      <div v-for="city in cityList" @click="parseCity(city)" class="city-result">
        <img :src="'https://flagsapi.com/' + city.country + '/flat/64.png'" class="country-flag-icon">
        <span class="city-result-name">{{ city.name.replaceAll('"', "") }}</span>
        <span class="city-result-state-country">{{ city.state }}, {{city.country}}</span>
      </div>
    </div>

    <div class="current-weather">
      <span class="current-weather-text">Зараз</span>
      <hr class="separator" />

      <div class="weather-details">
        <div class="current-temp">{{ parseInt(currentWeather.temp) }}°C</div>
        <img class="current-weather-icon" :src="'src/assets/weather_icons/' + currentWeather.icon + '.png'" />
        <div class="weather-description">{{ currentWeather.weather_name }}</div>
      </div>

      <div class="location-details">
        <div class="current-date">
          {{ currentWeather.date[2] }} {{ currentWeather.date[0] }},
          {{ currentWeather.date[3] }}
        </div>
        <div class="location">{{ cityName }}, {{ countryName }}</div>
        <img src="./assets/icons/calendar.svg" class="calendar-icon" />
        <img src="./assets/icons/my_location.svg" class="location-icon" />
      </div>
    </div>

    <div class="five-day-forecast">
      <div v-for="forecast in fiveDayForecast.forecast" class="day">
        <div class="forecast-temp">{{ parseInt(forecast.temp) }}°</div>
        <div class="forecast-date">
          {{ forecast.date[0] }} {{ forecast.date[3] }}
        </div>
        <div class="forecast-day">{{ forecast.date[1] }}</div>
        <img class="forecast-icon" :src="'src/assets/weather_icons/' + forecast.icon + '.png'" />
      </div>
    </div>

    <span class="forecast-text">Прогноз на 5 днів</span>
    <span class="today-at-text">Сьогодні в</span>

    <div class="weather-surface">
      <span class="weather-conditions-text">Погодні умови</span>
      <div class="weather-detail surf1">
        <span class="air-quality-index">Індекс якості повітря</span>
        <div class="index">{{ airPollution.status }}</div>
        <img src="./assets/icons/wind.svg" class="air-index-icon" />
        <span class="pm">PM25</span>
        <span class="so">SO2</span>
        <span class="no">NO2</span>
        <span class="o3">O3</span>
        <span class="pm_ value">{{ airPollution.pm25 }}</span>
        <span class="so_ value">{{ airPollution.so2 }}</span>
        <span class="no_ value">{{ airPollution.no2 }}</span>
        <span class="o3_ value">{{ airPollution.o3 }}</span>
      </div>

      <div class="weather-detail surf2">
        <span class="sunrise-sunset">Схід та Захід</span>
        <span class="sunrise-text">Схід</span>
        <span class="sunset-text">Захід</span>
        <span class="rise-time">{{ currentWeather.sunrise }}</span>
        <span class="set-time">{{ currentWeather.sunset }}</span>
        <img src="./assets/icons/sun.svg" class="rise-time-icon" />
        <img src="./assets/icons/moon.svg" class="set-time-icon" />
      </div>

      <div class="weather-detail surf3">
        <span class="pressure-text">Тиск</span>
        <span class="pressure-value">{{ currentWeather.pressure }}hPa</span>
        <img src="./assets/icons/waves.svg" class="pressure-icon" />
      </div>

      <div class="weather-detail surf4">
        <span class="humidity-text">Вологість</span>
        <span class="humidity-value">{{ currentWeather.humidity }}%</span>
        <img src="./assets/icons/humidity.svg" class="humidity-icon" />
      </div>

      <div class="weather-detail surf5">
        <span class="visibility-text">Видимість</span>
        <span class="visibility-value">{{ currentWeather.visibility }}km</span>
        <img src="./assets/icons/eye.svg" class="visibility-icon" />
      </div>

      <div class="weather-detail surf6">
        <span class="feels-like-text">Відчувається як</span>
        <span class="feels-like-value">{{ parseInt(currentWeather.feelslike) }}°</span>
        <img src="./assets/icons/temp.svg" class="temp-icon" />
      </div>
    </div>

    <div class="hourly-forecast">
      <div v-for="hourlyForecast in hourlyForecast.forecast" class="hourly-weather">
        <div class="hourly-temp">{{ parseInt(hourlyForecast.temp) }}°</div>
        <img class="hourly-icon" :src="'src/assets/weather_icons/' + hourlyForecast.icon + '.png'" />
        <div class="hourly-time">{{ hourlyForecast.time }}</div>
      </div>
    </div>
  </div>

  
</template>