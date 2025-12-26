use leptos::prelude::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WeatherResponse {
    location: Location,
    current: CurrentWeather,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    #[serde(rename = "tz_id")]
    timezone: String,
    #[serde(rename = "localtime_epoch")]
    localtime_epoch: i64,
    localtime: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CurrentWeather {
    #[serde(rename = "last_updated_epoch")]
    last_updated_epoch: i64,
    #[serde(rename = "last_updated")]
    last_updated: String,
    #[serde(rename = "temp_c")]
    temp_c: f64,
    #[serde(rename = "temp_f")]
    temp_f: f64,
    #[serde(rename = "is_day")]
    is_day: i32,
    condition: Condition,
    #[serde(rename = "wind_kph")]
    wind_kph: f64,
    #[serde(rename = "wind_degree")]
    wind_degree: i32,
    #[serde(rename = "wind_dir")]
    wind_dir: String,
    #[serde(rename = "pressure_mb")]
    pressure_mb: f64,
    #[serde(rename = "precip_mm")]
    precip_mm: f64,
    humidity: i32,
    cloud: i32,
    #[serde(rename = "feelslike_c")]
    feelslike_c: f64,
    #[serde(rename = "vis_km")]
    visibility_km: f64,
    uv: f64,
    #[serde(rename = "gust_kph")]
    gust_kph: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Condition {
    text: String,
    icon: String,
    code: i32,
}

#[component]
fn WeatherDashboard() -> impl IntoView {
    let (city, set_city) = signal("Kolkata".to_string());
    let (weather_data, set_weather_data) = signal::<Option<WeatherResponse>>(None);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);
    
    // Your API key
    let api_key = "b98ed768b2c648a58c554303252412";
    
    // Fetch real weather data
    let fetch_weather = move || {
        let city_name = city.get();
        let api_key_clone = api_key.to_string();
        
        set_loading.set(true);
        set_error.set(None);
        
        spawn_local(async move {
            match fetch_weather_data(&api_key_clone, &city_name).await {
                Ok(data) => {
                    set_weather_data.set(Some(data));
                    set_error.set(None);
                }
                Err(err) => {
                    set_error.set(Some(err));
                    set_weather_data.set(None);
                }
            }
            set_loading.set(false);
        });
    };
    
    // Load initial weather
    Effect::new(move |_| {
        fetch_weather();
    });
    
    // Handle search
    let search_weather = move |_| {
        fetch_weather();
    };
    
    view! {
        <div class="dashboard-container">
            <div class="header">
                <h1>"Live Weather Dashboard"</h1>
                <p>"Real-time weather from WeatherAPI.com"</p>
            </div>
            
            <div class="search-section">
                <div class="search-box">
                    <input
                        type="text"
                        class="search-input"
                        placeholder="Enter city name"
                        prop:value=city
                        on:input=move |ev| set_city.set(event_target_value(&ev))
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                fetch_weather();
                            }
                        }
                    />
                    <button class="search-btn" on:click=search_weather>
                        <i class="fas fa-search"></i>
                        " Search"
                    </button>
                </div>
                
                <div class="sample-cities">
                    <span style="color: white; margin-right: 10px;">"Quick cities: "</span>
                    {["Kolkata", "Delhi", "Mumbai", "London", "New York", "Tokyo"]
                        .into_iter()
                        .map(|c| {
                            let city_name = c.to_string();
                            view! {
                                <button
                                    class="city-btn"
                                    on:click=move |_| {
                                        set_city.set(city_name.clone());
                                        fetch_weather();
                                    }
                                >
                                    {c}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
            
            // Show error if any
            {move || error.get().map(|err| view! {
                <div class="error-message">
                    <i class="fas fa-exclamation-triangle"></i>
                    {format!("Error: {}", err)}
                </div>
            })}
            
            // Show loading or weather data
            <Show
                when=move || loading.get()
                fallback=move || {
                    view! {
                        <Show
                            when=move || weather_data.get().is_some()
                            fallback=move || view! { 
                                <div class="loading-spinner">
                                    <i class="fas fa-cloud"></i>
                                    <p>"Search for a city to see weather"</p>
                                </div>
                            }
                        >
                            {move || weather_data.get().map(|weather| view! {
                                <WeatherDisplay weather=weather/>
                            })}
                        </Show>
                    }
                }
            >
                <div class="loading-spinner">
                    <i class="fas fa-spinner fa-spin"></i>
                    <p>"Fetching live weather..."</p>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn WeatherDisplay(weather: WeatherResponse) -> impl IntoView {
    let temp = weather.current.temp_c;
    let temp_color = if temp > 30.0 {
        "#f46b45"
    } else if temp < 10.0 {
        "#667eea"
    } else {
        "#eea849"
    };
    
    // Get weather icon
    let weather_icon = get_weather_icon(&weather.current.condition.text, weather.current.is_day);
    
    // Format time
    let local_time = weather.location.localtime.clone();
    let display_time = if local_time.len() > 10 {
        local_time[11..].to_string()
    } else {
        local_time
    };
    
    view! {
        <div class="weather-cards">
            <div class="weather-card">
                <div class="card-header">
                    <h2>"Current Weather"</h2>
                    <div class="time">{display_time}</div>
                </div>
                
                <div class="current-weather">
                    <div class="weather-icon">
                        <i class=weather_icon style=move || format!("color: {}", temp_color)></i>
                    </div>
                    <div class="temperature" style=move || format!("color: {}", temp_color)>
                        {format!("{:.1}", temp)}
                        <span>"°C"</span>
                    </div>
                    <div class="weather-condition">
                        {weather.current.condition.text.clone()}
                    </div>
                    <div class="location">
                        {format!("{}, {}, {}", weather.location.name, weather.location.region, weather.location.country)}
                    </div>
                </div>
                
                <div class="weather-details">
                    <div class="detail-item">
                        <div class="detail-icon">
                            <i class="fas fa-temperature-high"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"Feels Like"</h4>
                            <p>{format!("{:.1}°C", weather.current.feelslike_c)}</p>
                        </div>
                    </div>
                    
                    <div class="detail-item">
                        <div class="detail-icon">
                            <i class="fas fa-tint"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"Humidity"</h4>
                            <p>{format!("{}%", weather.current.humidity)}</p>
                        </div>
                    </div>
                    
                    <div class="detail-item">
                        <div class="detail-icon">
                            <i class="fas fa-wind"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"Wind Speed"</h4>
                            <p>{format!("{} km/h", weather.current.wind_kph)}</p>
                        </div>
                    </div>
                    
                    <div class="detail-item">
                        <div class="detail-icon">
                            <i class="fas fa-compress-alt"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"Pressure"</h4>
                            <p>{format!("{} mb", weather.current.pressure_mb)}</p>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="weather-card">
                <div class="card-header">
                    <h2>"Additional Details"</h2>
                    <div class="time">"Live Data"</div>
                </div>
                
                <div class="weather-details">
                    <div class="detail-item">
                        <div class="detail-icon" style="background: linear-gradient(135deg, #00dbde, #fc00ff);">
                            <i class="fas fa-wind"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"Wind Direction"</h4>
                            <p>{weather.current.wind_dir.clone()}</p>
                        </div>
                    </div>
                    
                    <div class="detail-item">
                        <div class="detail-icon" style="background: linear-gradient(135deg, #8a2387, #f27121);">
                            <i class="fas fa-sun"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"UV Index"</h4>
                            <p>{format!("{:.1}", weather.current.uv)}</p>
                        </div>
                    </div>
                    
                    <div class="detail-item">
                        <div class="detail-icon" style="background: linear-gradient(135deg, #f46b45, #eea849);">
                            <i class="fas fa-eye"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"Visibility"</h4>
                            <p>{format!("{} km", weather.current.visibility_km)}</p>
                        </div>
                    </div>
                    
                    <div class="detail-item">
                        <div class="detail-icon" style="background: linear-gradient(135deg, #667eea, #764ba2);">
                            <i class="fas fa-cloud"></i>
                        </div>
                        <div class="detail-info">
                            <h4>"Cloud Cover"</h4>
                            <p>{format!("{}%", weather.current.cloud)}</p>
                        </div>
                    </div>
                </div>
                
                <div class="extra-info">
                    <div class="extra-info-item">
                        <span class="extra-info-label">"Coordinates"</span>
                        <span class="extra-info-value">
                            {format!("{:.2}°, {:.2}°", weather.location.lat, weather.location.lon)}
                        </span>
                    </div>
                    <div class="extra-info-item">
                        <span class="extra-info-label">"Last Updated"</span>
                        <span class="extra-info-value">
                            {weather.current.last_updated.clone()}
                        </span>
                    </div>
                    <div class="extra-info-item">
                        <span class="extra-info-label">"Gust Speed"</span>
                        <span class="extra-info-value">
                            {format!("{} km/h", weather.current.gust_kph)}
                        </span>
                    </div>
                    <div class="extra-info-item">
                        <span class="extra-info-label">"Precipitation"</span>
                        <span class="extra-info-value">
                            {format!("{} mm", weather.current.precip_mm)}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}

async fn fetch_weather_data(api_key: &str, city: &str) -> Result<WeatherResponse, String> {
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        api_key, city
    );
    
    let response = reqwest::get(&url).await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }
    
    let weather_data = response.json::<WeatherResponse>().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    Ok(weather_data)
}

fn get_weather_icon(condition: &str, is_day: i32) -> String {
    let condition_lower = condition.to_lowercase();
    
    if condition_lower.contains("sunny") {
        "fas fa-sun".to_string()
    } else if condition_lower.contains("clear") {
        "fas fa-star".to_string()
    } else if condition_lower.contains("cloud") {
        "fas fa-cloud".to_string()
    } else if condition_lower.contains("rain") {
        "fas fa-cloud-rain".to_string()
    } else if condition_lower.contains("drizzle") {
        "fas fa-cloud-rain".to_string()
    } else if condition_lower.contains("thunder") {
        "fas fa-bolt".to_string()
    } else if condition_lower.contains("snow") {
        "fas fa-snowflake".to_string()
    } else if condition_lower.contains("mist") || condition_lower.contains("fog") {
        "fas fa-smog".to_string()
    } else if condition_lower.contains("haze") {
        "fas fa-smog".to_string()
    } else {
        "fas fa-cloud".to_string()
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Live Weather Dashboard"/>
        <WeatherDashboard/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
