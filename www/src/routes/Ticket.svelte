
<script lang="ts">
</script>

<div class="mx-4 ticket">
  <div class="ticket_content">
    <slot />
  </div>
</div>

<style lang="postcss">
.ticket {
  position: relative;
  box-sizing: border-box;
  width: 300px;
  height: 450px;
  padding: 20px;
  border-radius: 10px;
  background: #FBFBFB;
  box-shadow: 0 0 200px -80px theme('colors.orange.500');
}

.ticket::before,
.ticket::after {
  content: '';
  position: absolute;
  left: 5px;
  height: 6px;
  width: 290px;
}

.ticket::before {
  top: -5px;
  background: radial-gradient(circle, transparent, transparent 50%, #FBFBFB 50%, #FBFBFB 100%) -7px -8px / 16px 16px repeat-x;
}

.ticket::after {
  bottom: -5px;
  background: radial-gradient(circle, transparent, transparent 50%, #FBFBFB 50%, #FBFBFB 100%) -7px -2px / 16px 16px repeat-x;
}

.ticket_content {
  box-sizing: border-box;
  height: 100%;
  width: 100%;
}
</style>
