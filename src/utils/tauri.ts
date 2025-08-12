import { invoke } from '@tauri-apps/api/core'

export async function readAndParseMultipleFrontmatter(filePaths: string[]): Promise<Record<string, Record<string, unknown>>> {
  return await invoke('read_and_parse_multiple_frontmatter', { filePaths })
}

export async function getAllFilesOfDir(folderPath: string): Promise<string[]> {
  return await invoke('get_all_files_of_dir', { dir: folderPath })
}

export async function readAndParseYamlFrontmatter(filePath: string): Promise<Record<string, unknown>> {
  return await invoke('read_and_parse_yaml_frontmatter', { filePath })
}

export async function exportFrontmatterToXlsx(data: Record<string, Record<string, unknown>>, outputPath: string): Promise<string> {
  return await invoke('export_frontmatter_to_xlsx', { data, outputPath })
}

export async function importFrontmatterFromXlsx(xlsxPath: string, basePath: string): Promise<string> {
  return await invoke('import_frontmatter_from_xlsx', { xlsxPath, basePath })
}

export async function writeMultipleFrontmatter(fileData: Record<string, Record<string, unknown>>): Promise<Record<string, boolean>> {
  return await invoke('write_multiple_frontmatter', { fileData })
}

export async function backupFilesAsTarZst(fileList: string[], basePath: string, outputPath: string, zstdLevel: number): Promise<string> {
  return await invoke('backup_files_as_tar_zst', { fileList, basePath, outputPath, zstdLevel })
}

export async function restoreFilesFromTarZst(backupPath: string, restoreBasePath: string): Promise<{ success_count: number, failed_count: number, failed_files: string[] }> {
  return await invoke('restore_files_from_tar_zst', { backupPath, restoreBasePath })
}
