<script setup lang="ts">
import type { Highlighter } from 'shiki/bundle/web'
import { createHighlighter } from 'shiki/bundle/web'
import { nextTick, onMounted, ref, watch } from 'vue'
import { useLanguage } from '../../composables/useLanguage'

const props = withDefaults(defineProps<Props>(), {
  readonly: false,
  theme: 'github-light',
  placeholder: '',
})

const emit = defineEmits<Emits>()

const { t } = useLanguage()

interface Props {
  modelValue: string
  readonly?: boolean
  theme?: string
  placeholder?: string
}

interface Emits {
  (e: 'update:modelValue', value: string): void
  (e: 'change', value: string): void
  (e: 'error', error: string | null): void
}

const editorContainer = ref<HTMLDivElement>()
const textArea = ref<HTMLTextAreaElement>()
const highlightLayer = ref<HTMLDivElement>()
const highlighter = ref<Highlighter>()
const isComposing = ref(false)
const errorMessage = ref<string | null>(null)

// 初始化Shiki高亮器
onMounted(async () => {
  try {
    highlighter.value = await createHighlighter({
      themes: [props.theme, 'github-dark'],
      langs: ['json'],
    })
    await updateHighlight()
  }
  catch (error) {
    console.error('Failed to initialize Shiki highlighter:', error)
  }
})

// 验证JSON格式
function validateJSON(value: string): string | null {
  if (!value.trim())
    return null

  try {
    JSON.parse(value)
    return null
  }
  catch (error) {
    return error instanceof Error ? error.message : 'Invalid JSON format'
  }
}

// 更新高亮显示
async function updateHighlight() {
  if (!highlighter.value || !highlightLayer.value)
    return

  const value = props.modelValue || ''
  const error = validateJSON(value)
  errorMessage.value = error
  emit('error', error)

  try {
    const html = highlighter.value.codeToHtml(value || ' ', {
      lang: 'json',
      theme: props.theme,
    })

    // 提取高亮的代码部分
    const codeMatch = html.match(/<code[^>]*>([\s\S]*?)<\/code>/)
    if (codeMatch) {
      highlightLayer.value.innerHTML = codeMatch[1]
    }
  }
  catch (error) {
    console.error('Highlighting error:', error)
  }
}

// 处理输入事件
function handleInput(event: Event) {
  if (isComposing.value)
    return

  const target = event.target as HTMLTextAreaElement
  const value = target.value
  emit('update:modelValue', value)
  emit('change', value)
}

// 处理组合输入开始
function handleCompositionStart() {
  isComposing.value = true
}

// 处理组合输入结束
function handleCompositionEnd(event: CompositionEvent) {
  isComposing.value = false
  const target = event.target as HTMLTextAreaElement
  const value = target.value
  emit('update:modelValue', value)
  emit('change', value)
}

// 同步滚动
function handleScroll() {
  if (!textArea.value || !highlightLayer.value)
    return

  highlightLayer.value.scrollTop = textArea.value.scrollTop
  highlightLayer.value.scrollLeft = textArea.value.scrollLeft
}

// 处理Tab键缩进
function handleKeyDown(event: KeyboardEvent) {
  if (event.key === 'Tab') {
    event.preventDefault()

    const textarea = event.target as HTMLTextAreaElement
    const start = textarea.selectionStart
    const end = textarea.selectionEnd
    const value = textarea.value

    // 插入两个空格作为缩进
    const newValue = `${value.substring(0, start)}  ${value.substring(end)}`

    emit('update:modelValue', newValue)

    // 恢复光标位置
    nextTick(() => {
      textarea.selectionStart = textarea.selectionEnd = start + 2
    })
  }
}

// 格式化JSON
function formatJSON() {
  try {
    const parsed = JSON.parse(props.modelValue)
    const formatted = JSON.stringify(parsed, null, 2)
    emit('update:modelValue', formatted)
  }
  catch (error) {
    return error
  }
}

// 监听modelValue变化，更新高亮
watch(() => props.modelValue, updateHighlight, { immediate: true })

// 监听主题变化
watch(() => props.theme, updateHighlight)
</script>

<template>
  <div class="json-editor w-full">
    <!-- 工具栏 -->
    <div class="toolbar flex items-center justify-between border-b border-gray-200 p-2">
      <div class="flex items-center gap-2">
        <span class="text-sm text-gray-700 font-medium">{{ t('jsonEditor.title') }}</span>
        <button
          v-if="!readonly"
          class="rounded bg-blue-500 px-3 py-1 text-xs text-white transition-colors hover:bg-blue-600"
          :disabled="!!errorMessage"
          @click="formatJSON"
        >
          {{ t('jsonEditor.format') }}
        </button>
      </div>

      <!-- 错误信息 -->
      <div v-if="errorMessage" class="flex items-center gap-1 text-xs text-red-500">
        <span class="i-ri-error-warning-line" />
        {{ errorMessage }}
      </div>

      <!-- 状态指示 -->
      <div v-else class="flex items-center gap-1 text-xs text-green-500">
        <span class="i-ri-check-line" />
        {{ t('jsonEditor.validJson') }}
      </div>
    </div>

    <!-- 编辑器容器 -->
    <div
      ref="editorContainer"
      class="editor-container relative overflow-hidden"
      :class="{ 'border-red-300': errorMessage, 'border-gray-300': !errorMessage }"
    >
      <!-- 高亮层 -->
      <div
        ref="highlightLayer"
        class="highlight-layer pointer-events-none absolute inset-0 overflow-auto whitespace-pre p-3 text-sm leading-6 font-mono"
        :class="{ 'opacity-50': errorMessage }"
      />

      <!-- 文本输入层 -->
      <textarea
        ref="textArea"
        :value="modelValue"
        :readonly="readonly"
        :placeholder="placeholder || t('files.jsonPlaceholder')"
        class="textarea absolute inset-0 h-full w-full resize-none border-0 bg-transparent p-3 text-sm text-transparent leading-6 font-mono caret-black focus:outline-none"
        :class="{ 'cursor-not-allowed': readonly }"
        @input="handleInput"
        @scroll="handleScroll"
        @keydown="handleKeyDown"
        @compositionstart="handleCompositionStart"
        @compositionend="handleCompositionEnd"
      />
    </div>
  </div>
</template>

<style scoped>
.json-editor {
  @apply border border-gray-300 rounded-lg bg-white shadow-sm;
  height: 400px;
  display: flex;
  flex-direction: column;
}

.editor-container {
  flex: 1;
  min-height: 0;
  border-top: 1px solid #e5e7eb;
}

.highlight-layer {
  z-index: 1;
}

.textarea {
  z-index: 2;
}

.textarea::selection {
  background-color: rgba(59, 130, 246, 0.3);
}

/* 确保Shiki生成的代码样式正确显示 */
.highlight-layer :deep(span) {
  font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
}

/* 滚动条样式 */
.highlight-layer::-webkit-scrollbar,
.textarea::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.highlight-layer::-webkit-scrollbar-track,
.textarea::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.highlight-layer::-webkit-scrollbar-thumb,
.textarea::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

.highlight-layer::-webkit-scrollbar-thumb:hover,
.textarea::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style>
