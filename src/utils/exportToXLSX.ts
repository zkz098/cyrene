import type { useFilesStore } from '../stores/useFilesStore'
import { save } from '@tauri-apps/plugin-dialog'
import { exportFrontmatterToXlsx } from './tauri'

export async function exportToXLSX(filesStore: ReturnType<typeof useFilesStore>) {
  const selected = await save({
    title: '请选择导出文件的保存位置',
    filters: [
      {
        name: 'Excel 文件',
        extensions: ['xlsx'],
      },
    ],
    defaultPath: `frontmatter-export-${new Date().toISOString().split('T')[0]}.xlsx`,
  })

  const temp = {} as Record<string, Record<string, unknown>>

  filesStore.getFileAbsolutePathList().forEach((key) => {
    temp[filesStore.files[key].relativePath] = filesStore.files[key].frontmatter
  })

  if (selected) {
    await exportFrontmatterToXlsx(temp, selected)
  }
}
