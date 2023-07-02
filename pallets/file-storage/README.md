The File Storage Pallet includes the following functions:

upload_file: This function allows an account to upload a file to the decentralized cloud platform. It validates the file size against the maximum allowed size (MaxFileSize) specified in the configuration trait. It creates a FileInfo struct to store the file metadata and inserts it into the Files storage map.

download_file: This function allows an account to download a file based on its content address. It retrieves the file information from the Files storage map and emits an event to indicate that the file has been downloaded.

delete_file: This function allows the root (admin) account to delete a file from the decentralized cloud platform. It removes the file information from the Files storage map.

The FileInfo struct stores metadata about the file, such as the file name, size, timestamp, uploader's account ID, and content address.

## Here's an explanation of the code functionality:

Configuration and Dependencies:

The Config trait defines the configuration for the File Storage Pallet. It requires the implementation of various associated types and constants, including the event type (Event), the maximum file size (MaxFileSize), and the balance type (Balance).
The FileInfo struct represents the metadata of a file. It includes fields such as the file name, size, timestamp, uploader's account ID, and content address.
Storage:

The Files storage map is used to store the file metadata. It maps the content address of a file to its corresponding FileInfo.
Events:

The Event enum defines the events emitted by the File Storage Pallet. It includes events such as FileUploaded, FileDownloaded, and FileDeleted. These events are used to notify other parts of the system about file-related actions.
Errors:

The Error enum defines the possible errors that can occur during file operations. In this case, there are two errors: FileNotFound and FileSizeExceeded.
Dispatchable Functions:

upload_file: This function allows an account to upload a file to the decentralized cloud platform. It verifies that the file size is within the allowed limit (MaxFileSize). It creates a FileInfo struct with the provided file metadata and inserts it into the Files storage map. An event (FileUploaded) is emitted to notify listeners about the file upload.
download_file: This function allows an account to download a file based on its content address. It retrieves the file information from the Files storage map and emits an event (FileDownloaded) to indicate that the file has been downloaded.
delete_file: This function allows the root (admin) account to delete a file from the decentralized cloud platform. It checks if the file exists in the Files storage map and removes it. An event (FileDeleted) is emitted to indicate that the file has been deleted.
The code provides the basic functionality for storing, retrieving, and deleting files in the decentralized cloud platform. 