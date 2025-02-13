<script setup lang="ts">
import { reactive, ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Create a configuration object with only the most important options.
const config = reactive({
  stapmLimit: 45000,             // Sustained Power Limit (mW)
  fastLimit: 50000,              // Actual Power Limit (PPT FAST) (mW)
  slowLimit: 40000,              // Average Power Limit (PPT SLOW) (mW)
  tctlTemp: 85,                  // Tctl Temperature Limit (°C)
  vrmCurrent: 1000,              // VRM Current Limit (TDC VDD) (mA)
  vrmmaxCurrent: 1500,           // VRM Maximum Current Limit (EDC VDD) (mA)
  prochotDeassertionRamp: 10,    // Prochot Deassertion Ramp (unit as required)
  powerSaving: false,            // Enable power-saving behavior
  maxPerformance: false          // Enable max-performance behavior
});

// For preset functionality.
const presetName = ref("");
const presets = ref<Array<{ name: string; [key: string]: unknown }>>([]);

// Load any saved presets from localStorage when the component is mounted.
onMounted(() => {
  const stored = localStorage.getItem("amdadj_presets");
  if (stored) {
    try {
      presets.value = JSON.parse(stored);
    } catch (error) {
      console.error("Failed to parse stored presets:", error);
    }
  }
});

// Call the Tauri command to apply the current AMDadj configuration.
function applyConfiguration() {
  console.log("Applying AMDadj configuration:", { ...config });
  invoke("apply_amdadj_config", { ...config })
      .then((result) => {
        console.log("Configuration applied:", result);
      })
      .catch((error) => {
        console.error("Error applying configuration:", error);
      });
}

// Save the current configuration as a preset.
function savePreset() {
  if (!presetName.value.trim()) {
    alert("Please enter a preset name.");
    return;
  }
  const newPreset = { name: presetName.value.trim(), ...config };
  presets.value.push(newPreset);
  localStorage.setItem("amdadj_presets", JSON.stringify(presets.value));
  presetName.value = "";
}

// Load a preset into the configuration.
function loadPreset(preset: { name: string; [key: string]: unknown }) {
  for (const key in preset) {
    if (key !== "name" && Object.prototype.hasOwnProperty.call(config, key)) {
      (config as any)[key] = preset[key];
    }
  }
}
</script>

<template>
  <main class="container">
    <h1>AMdadj: Super Important Configuration</h1>

    <!-- Power Limits Section -->
    <section class="config-section">
      <h2>Power Limits (mW)</h2>
      <div class="form-group">
        <label for="stapmLimit">Sustained Power Limit</label>
        <input id="stapmLimit" type="number" v-model.number="config.stapmLimit" />
      </div>
      <div class="form-group">
        <label for="fastLimit">Actual Power Limit</label>
        <input id="fastLimit" type="number" v-model.number="config.fastLimit" />
      </div>
      <div class="form-group">
        <label for="slowLimit">Average Power Limit</label>
        <input id="slowLimit" type="number" v-model.number="config.slowLimit" />
      </div>
    </section>

    <!-- Temperature Setting -->
    <section class="config-section">
      <h2>Temperature</h2>
      <div class="form-group">
        <label for="tctlTemp">Tctl Temperature Limit (°C)</label>
        <input id="tctlTemp" type="number" v-model.number="config.tctlTemp" />
      </div>
    </section>

    <!-- Current Limits Section -->
    <section class="config-section">
      <h2>Current Limits (mA)</h2>
      <div class="form-group">
        <label for="vrmCurrent">VRM Current Limit (TDC VDD)</label>
        <input id="vrmCurrent" type="number" v-model.number="config.vrmCurrent" />
      </div>
      <div class="form-group">
        <label for="vrmmaxCurrent">VRM Maximum Current Limit (EDC VDD)</label>
        <input id="vrmmaxCurrent" type="number" v-model.number="config.vrmmaxCurrent" />
      </div>
    </section>

    <!-- Other Settings Section -->
    <section class="config-section">
      <h2>Other Settings</h2>
      <div class="form-group">
        <label for="prochotDeassertionRamp">Prochot Deassertion Ramp</label>
        <input id="prochotDeassertionRamp" type="number" v-model.number="config.prochotDeassertionRamp" />
      </div>
      <div class="form-group checkbox-group">
        <label>
          <input type="checkbox" v-model="config.powerSaving" />
          Enable Power Saving
        </label>
      </div>
      <div class="form-group checkbox-group">
        <label>
          <input type="checkbox" v-model="config.maxPerformance" />
          Enable Max Performance
        </label>
      </div>
    </section>

    <div class="buttons">
      <button @click="applyConfiguration">Apply Configuration</button>
    </div>

    <!-- Preset Section -->
    <section class="preset-section">
      <h2>Presets</h2>
      <div class="preset-form">
        <input type="text" v-model="presetName" placeholder="Preset name" />
        <button @click="savePreset">Save Preset</button>
      </div>
      <ul v-if="presets.length">
        <li v-for="(preset, idx) in presets" :key="idx">
          <span>{{ preset.name }}</span>
          <button @click="loadPreset(preset)">Load Preset</button>
        </li>
      </ul>
    </section>
  </main>
</template>

<style scoped>
.container {
  margin: 0 auto;
  max-width: 600px;
  padding: 2rem;
  text-align: center;
}
h1, h2 {
  margin-bottom: 1rem;
}
.config-section, .preset-section {
  margin-bottom: 2rem;
  border: 1px solid #ccc;
  padding: 1rem;
  border-radius: 8px;
}
.form-group {
  margin-bottom: 1rem;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
.form-group label {
  margin-bottom: 0.4rem;
  font-weight: bold;
}
.form-group input[type="number"],
.form-group input[type="text"] {
  width: 100%;
  padding: 0.5rem;
  font-size: 1rem;
}
.checkbox-group {
  flex-direction: row;
  align-items: center;
}
.buttons {
  margin-top: 1rem;
}
button {
  padding: 0.5rem 1rem;
  font-size: 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  background-color: #646cff;
  color: #ffffff;
  margin: 0.5rem;
}
button:hover {
  background-color: #535bf2;
}
ul {
  list-style: none;
  padding: 0;
}
ul li {
  margin: 0.5rem 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}
</style>
