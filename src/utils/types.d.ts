export interface postFile {
  relativePath: string;
  frontmatter: Record<string, any>;
  modified: boolean;
  // absolutePath?: string; // Optional, used for file operations
}