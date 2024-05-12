<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const city_search = ref("Lviv");
const city_name = ref("");
const city_country = ref("");

const now = ref({"date": ["", "", "", "", ""], "temp": 0.0, "feelslike": 0.0, "pressure": 0, "humidity": 0, "visibility": 0, "sunrise": "HH:MM", "sunset": "HH:MM"});
const hourlyforecast = ref({"forecast": [{"time": "HH:MM", "temp": 0.0}]});
const fivedayforecast = ref({"forecast": [{"date": ["", "", "", "", ""], "temp": 0.0}]});



async function search_city() {
	let city = city_search.value;

  let json = await invoke('search_city', {name: city}).then((data) => {
    return JSON.parse(data.replaceAll("'", '"')).result;
  });

  city_country.value = json[0].country;
  city_name.value = json[0].name;

  parse_weather(json[0].lat, json[0].lon);
  get_airindex(json[0].lat, json[0].lon);
}

async function parse_weather(lat, lon) {
  let json = await invoke('get_weather', {lat: lat, lon: lon}).then((data) => {
    return JSON.parse(data.replaceAll("'", '"'));
  });

  now.value = json.now;
  hourlyforecast.value = json.hourlyforecast;
  fivedayforecast.value = json.fivedayforecast;
  console.log(now.value, hourlyforecast.value, fivedayforecast.value);
}

async function get_airindex(lat, lon) {
  let json = await invoke('get_airindex', {lat: lat, lon: lon}).then((data) => {
    return JSON.parse(data.replaceAll("'", '"'));
  });

  console.log(json);

}


</script>

<template>
  <div class="container">

    <div class="topbar">
      <span class="appname">JustWeather</span>
      <div class="searchbar">
        <input class="search" v-model="city_search" v-on:keyup.enter="search_city()" placeholder="Шукати місто.." type="text">
      </div>
    </div>

    <div class="now">
      <span class="nowtxt">Зараз</span>
      <hr class="hr">
      <div class="weathernow">
        <div class="tempnow">{{parseInt(now.temp)}}°C</div>
        <img class="iconnow" :src="'src/assets/weather_icons/' + now.icon + '.png'">
        <div class="weathername">{{now.weather_name}}</div>
      </div>
      <div class="destination">
        <div class="datenow">{{now.date[2]}} {{now.date[0]}}, {{now.date[3]}}</div>
        <div class="place">{{city_name}}, {{city_country}}</div>
        <img src="" class="calendar">
        <img src="" class="">
      </div>
    </div>

    <div class="nextforecast">
      <div v-for="nf in fivedayforecast.forecast" class="day">
        <div class="tempnext">{{parseInt(nf.temp)}}°</div>
        <div class="datenext">{{nf.date[0]}} {{nf.date[3]}}</div>
        <div class="daynext">{{nf.date[1]}}</div>
        <img class="iconfloat" :src="'src/assets/weather_icons/' + nf.icon + '.png'">
      </div>
    </div>

    <span class="forecasttxt">Прогноз на 5 днів</span>
    <span class="todayattxt">Сьогодні в</span>

    <div class="weathersurface" >
      <span class="weathercondtxt">Погодні умови</span>

      <div class="surf surf1">
        <span class="airindex">Індекс якості повітря</span>
        <div class="index">
			
        </div>
        <span class="pm">PM25</span>
        <span class="so">SO2</span>
        <span class="no">NO2</span>
        <span  class="o3">O3</span>
      </div>

      <div class="surf surf2">
        <span class="sunrisesunset">Схід та Захід</span>
        <span class="sunrisetxt">Схід</span>
        <span class="sunsettxt">Захід</span>
        <span class="risetime">{{now.sunrise}}</span>
        <span class="settime">{{now.sunset}}</span>
      </div>

      <div class="surf surf3">
        <span class="pressuretxt">Тиск</span>
        <span class="pressure">{{now.pressure}}hPa</span> 
      </div>

      <div class="surf surf4">
        <span class="humiditytxt">Вологість</span>
        <span class="humidity">{{now.humidity}}%</span> 
      </div>

      <div class="surf surf5">
        <span class="visibilitytxt">Видимість</span>
        <span class="visibility">{{now.visibility}}км</span> 
      </div>
      
      <div class="surf surf6">
        <span class="feelsliketxt">Відчувається як</span>
        <span class="feelslike">{{parseInt(now.feelslike)}}°</span> 
      </div>
    </div>

    <div class="hourly">
      <div v-for="h in hourlyforecast.forecast" class="hourlyweather">
        <div class="temphourly">{{parseInt(h.temp)}}°</div>
        <img class="hourlyicon" :src="'src/assets/weather_icons/' + h.icon + '.png'">
        <div class="timehourly">{{h.time}}</div>
      </div>
    </div> 

  </div>
</template>


