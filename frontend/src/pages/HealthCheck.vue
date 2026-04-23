<script setup lang="ts">
import { ref } from "vue";

const respContent = ref<string | undefined>(undefined);
const respStatus = ref<number | undefined>(undefined);
const checkHealth = async () => {
  const resp = await fetch("/api/health");
  const contentType = resp.headers.get("Content-Type");
  respStatus.value = resp.status;

  if (contentType?.includes("text/plain")) {
    respContent.value = await resp.text();
  }
};
</script>

<template>
  <main class="page-shell">
    <section class="hero">
      <p class="eyebrow">Health check</p>
      <h1>This page makes an API call to the back-end <code>/api/health</code></h1>
      <p class="lede">
        The button below will make a network call to the backend. This is handled by Axum.
      </p>
    </section>

    <section class="card response-panel">
      <h2>API response</h2>
      <div class="response-list">
        <div class="response-row">
          <p class="response-label">Status</p>
          <strong>{{ respStatus ?? "Not checked yet" }}</strong>
        </div>
        <div class="response-row">
          <p class="response-label">Content</p>
          <code class="response-code">{{ respContent ?? "No text response yet" }}</code>
        </div>
      </div>
      <form action="" @submit.prevent="checkHealth">
        <button type="submit">Check health</button>
      </form>
    </section>
  </main>
</template>

<style scoped>
.page-shell {
  width: min(100%, 1100px);
  margin: 0 auto;
  padding: 2.5rem 0 4rem;
}

.hero {
  margin-bottom: 2rem;
}

.eyebrow {
  margin: 0 0 0.75rem;
  font-size: 0.8rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  color: #385c37;
}

h1 {
  max-width: 14ch;
  margin: 0;
  font-size: clamp(2rem, 7vw, 4.5rem);
  line-height: 1;
  letter-spacing: -0.04em;
}

.lede {
  max-width: 42rem;
  margin: 1.25rem 0 0;
  font-size: 1.05rem;
  line-height: 1.6;
  color: #314033;
}

.card {
  padding: 1.25rem;
  border: 1px solid rgb(20 34 21 / 10%);
  border-radius: 1.25rem;
  background: rgb(255 255 255 / 72%);
  backdrop-filter: blur(10px);
  box-shadow: 0 18px 40px rgb(20 34 21 / 7%);
}

h2 {
  margin: 0 0 1rem;
  font-size: 1rem;
}

.response-panel {
  max-width: 44rem;
}

.response-list {
  display: grid;
  gap: 0.9rem;
  margin-bottom: 1.25rem;
}

.response-row {
  padding: 1rem 1.05rem;
  border-radius: 1rem;
  background: rgb(255 255 255 / 55%);
  border: 1px solid rgb(20 34 21 / 8%);
}

.response-label {
  margin-top: 0;
  margin-bottom: 0.4rem;
  font-size: 0.78rem;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: #5f7360;
}

strong {
  display: block;
  font-size: 1.1rem;
}

.response-code {
  display: block;
  color: #314033;
  line-height: 1.6;
  overflow-wrap: anywhere;
}

form {
  margin: 0;
}

button {
  border: 0;
  border-radius: 999px;
  padding: 0.8rem 1.15rem;
  font: inherit;
  font-weight: 700;
  color: #f4f6ef;
  background: #1d311f;
  cursor: pointer;
  transition:
    transform 150ms ease,
    box-shadow 150ms ease,
    background-color 150ms ease;
  box-shadow: 0 12px 30px rgb(20 34 21 / 14%);
}

button:hover {
  transform: translateY(-1px);
  background: #142215;
}

button:focus-visible {
  outline: 2px solid #385c37;
  outline-offset: 3px;
}

@media (max-width: 800px) {
  .page-shell {
    padding: 1.5rem 0 3rem;
  }
}
</style>
