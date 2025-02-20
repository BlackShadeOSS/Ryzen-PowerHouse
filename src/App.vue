<script setup lang="ts">
import { reactive, ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface Preset {
  name: string;
  stapm_limit?: number;
  fast_limit?: number;
  slow_limit?: number;
  slow_time?: number;
  tctl_temp?: number;
  apu_skin_temp?: number;
  dgpu_skin_temp?: number;
  power_saving: boolean;
  max_performance: boolean;
}

// The configuration object matching the Rust struct
const config = reactive<Preset>({
  name: "",
  stapm_limit: undefined,
  fast_limit: undefined,
  slow_limit: undefined,
  slow_time: undefined,
  tctl_temp: undefined,
  apu_skin_temp: undefined,
  dgpu_skin_temp: undefined,
  power_saving: false,
  max_performance: false,
});

// Presets management
const presetName = ref("");
const presets = ref<Preset[]>([]);

// Event listeners setup
onMounted(() => {
  // Load initial presets
  invoke("get_presets")
      .then((result: Preset[]) => {
        presets.value = result;
      })
      .catch((error) => {
        console.error("Error fetching presets:", error);
      });

  // Listen for tray events
  listen("tray-preset-selected", (event) => {
    const presetId = event.payload as string;
    const foundPreset = presets.value.find(p => p.name === presetId);

    if (foundPreset) {
      Object.assign(config, foundPreset);
    } else {
      console.error("Unknown preset selected from tray:", presetId);
    }
  });
});

async function applyConfiguration() {
  try {
    await invoke("apply_preset", config.name);
    console.log("Configuration applied successfully");
  } catch (error) {
    console.error("Error applying configuration:", error);
  }
}

async function savePreset() {
  // Validate preset name
  if (!presetName.value.trim()) {
    alert("Please enter a preset name.");
    return;
  }

  try {
    // Create new preset with current configuration
    const newPreset: Preset = { ...config };
    newPreset.name = presetName.value.trim();

    // Save preset to backend
    await invoke("save_preset", newPreset);

    // Clear preset name input
    presetName.value = "";

    // Refresh presets list
    const updatedPresets = await invoke<Preset[]>("get_presets");
    presets.value = updatedPresets;

    // Show success message
    console.log("Preset saved successfully");
  } catch (error) {
    console.error("Error saving preset:", error);
    alert("Failed to save preset: " + error);
  }
}

function loadPreset(preset: Preset) {
  Object.assign(config, preset);
}
</script>

<template>
  <main class="container">
    <h1>AMDadj Configuration</h1>

    <!-- Power Limits Section -->
    <section class="config-section">
      <h2>Power Limits (mW)</h2>
      <div class="form-group">
        <label for="stapmLimit">Sustained Power Limit</label>
        <input
            id="stapmLimit"
            type="number"
            v-model.number="config.stapm_limit"
            placeholder="Optional"
        />
      </div>
      <div class="form-group">
        <label for="fastLimit">Fast Power Limit</label>
        <input
            id="fastLimit"
            type="number"
            v-model.number="config.fast_limit"
            placeholder="Optional"
        />
      </div>
      <div class="form-group">
        <label for="slowLimit">Slow Power Limit</label>
        <input
            id="slowLimit"
            type="number"
            v-model.number="config.slow_limit"
            placeholder="Optional"
        />
      </div>
      <div class="form-group">
        <label for="slowTime">Slow Time</label>
        <input
            id="slowTime"
            type="number"
            v-model.number="config.slow_time"
            placeholder="Optional"
        />
      </div>
    </section>

    <!-- Temperature Settings -->
    <section class="config-section">
      <h2>Temperature Limits (°C)</h2>
      <div class="form-group">
        <label for="tctlTemp">TCTL Temperature Limit</label>
        <input
            id="tctlTemp"
            type="number"
            v-model.number="config.tctl_temp"
            placeholder="Optional"
        />
      </div>
      <div class="form-group">
        <label for="apuSkinTemp">APU Skin Temperature Limit</label>
        <input
            id="apuSkinTemp"
            type="number"
            v-model.number="config.apu_skin_temp"
            placeholder="Optional"
        />
      </div>
      <div class="form-group">
        <label for="dgpuSkinTemp">dGPU Skin Temperature Limit</label>
        <input
            id="dgpuSkinTemp"
            type="number"
            v-model.number="config.dgpu_skin_temp"
            placeholder="Optional"
        />
      </div>
    </section>

    <!-- Mode Selection -->
    <section class="config-section">
      <h2>Operating Modes</h2>
      <div class="mode-buttons">
        <button
            :class="{ active: config.power_saving }"
            @click="config.power_saving = !config.power_saving"
        >
          {{ config.power_saving ? 'Disable' : 'Enable' }} Power Saving
        </button>
        <button
            :class="{ active: config.max_performance }"
            @click="config.max_performance = !config.max_performance"
        >
          {{ config.max_performance ? 'Disable' : 'Enable' }} Max Performance
        </button>
      </div>
    </section>

    <div class="buttons">
      <button @click="applyConfiguration">Apply Configuration</button>
    </div>

    <!-- Preset Management -->
    <section class="preset-section">
      <h2>Presets</h2>

      <!-- Save New Preset -->
      <div class="preset-form">
        <input
            type="text"
            v-model="presetName"
            placeholder="Enter preset name"
        />
        <button @click="savePreset">Save Preset</button>
      </div>

      <!-- Preset List -->
      <ul v-if="presets.length > 0">
        <li v-for="(preset, idx) in presets" :key="idx">
          <span>{{ preset.name }}</span>
          <button @click="loadPreset(preset)">Load</button>
        </li>
      </ul>
      <p v-else>No presets saved yet.</p>
    </section>
  </main>
</template>

<style scoped>
.container {
  margin: 0 auto;
  max-width: 600px;
  padding: 2rem;
}

.config-section {
  border: 1px solid #ccc;
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: bold;
}

.form-group input[type="number"] {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.mode-buttons {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.buttons {
  margin-top: 1rem;
  text-align: center;
}

button {
  padding: 0.5rem 1rem;
  font-size: 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  background-color: #646cff;
  color: white;
}

button.active {
  background-color: #535bf2;
}

.preset-section {
  margin-top: 2rem;
}

.preset-form {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

ul {
  list-style: none;
  padding: 0;
}

li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  margin-bottom: 0.5rem;
}
</style>