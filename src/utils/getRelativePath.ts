export function getRelativePath(fullPath: string, basePath: string) {
  // 替换反斜杠为斜杠，统一处理
  const normalizedFull = fullPath.replace(/\\/g, '/')
  const normalizedBase = basePath.replace(/\\/g, '/')

  // 如果 base 是 full 的前缀，裁剪它
  if (normalizedFull.startsWith(normalizedBase)) {
    return normalizedFull.slice(normalizedBase.length).replace(/^\/?/, '/')
  }
  else {
    throw new Error('Base path is not a prefix of the full path.')
  }
}
