// 定义一个接口用于描述文件的属性
interface FileMeta {
  fileName: string
  fileType: string
  fileSize: number
  fileContent: string
}
export default FileMeta;

// Example：
//  const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement;
//    fileInput.addEventListener('change', async () => { 
//    const file = fileInput.files?.[0]; 
//    if (file) { const fileMeta = await getFileMeta(file); console.log(fileMeta); } });
// 读取文件方法
export function getFileMeta(file: File): Promise<FileMeta> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (event: ProgressEvent<FileReader>) => {
      const fileContent = event.target?.result as string;
      const type =  file.type
      const size = file.size
      const name = file.name
      console.log('fileContent')
      console.log(fileContent)
      resolve({
        fileName: name,
        fileType: type,
        fileSize: size,
        fileContent: fileContent
      });
    };
    reader.readAsText(file);
  });
}

export function getFile(fileMeta: FileMeta): File {
  const content = fileMeta.fileContent;
  const file = new File([content], fileMeta.fileName, { type: fileMeta.fileType, lastModified: Date.now() });
  return file;
}