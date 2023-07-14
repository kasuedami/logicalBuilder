use std::{path::{PathBuf, Path}, fmt, fs::{self}};

#[derive(Debug)]
pub struct Project {
    root: PathBuf,
    entries: ProjectEntry,
}

impl Project {
    pub fn create_new(root: &Path) -> Result<Self, CreateProjectError> {
        
        if !root.exists() {
            if fs::create_dir_all(root).is_err() {
                return Err(CreateProjectError)
            }
        }
        
        if !root.is_dir() {
            return Err(CreateProjectError)
        }

        if fs::read_dir(root).unwrap().next().is_some() {
            return Err(CreateProjectError)
        }
        
        let shapes_directory = root.join("shapes");

        if fs::create_dir(shapes_directory).is_err() {
            return Err(CreateProjectError)
        }

        let project = Self::from_root(root);

        match project {
            Ok(project) => Ok(project),
            Err(_) => Err(CreateProjectError),
        }
    }

    pub fn from_root(root: &Path) -> Result<Self, InvalidProject> {
        
        if root.is_dir() {
            
            let entries = Self::dir_to_project_entries(&root);

            Ok(Self {
                root: root.to_owned(),
                entries
            })

        } else {
            Err(InvalidProject)
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn entries(&self) -> &ProjectEntry {
        &self.entries
    }

    fn dir_to_project_entries(dir: &Path) -> ProjectEntry {

        let mut project_entries = vec![];

        for entry in fs::read_dir(dir).unwrap() {
            
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    let project_subdir = Self::dir_to_project_entries(&entry.path());
                    project_entries.push(project_subdir);
                } else {
                    project_entries.push(ProjectEntry::File(entry.path()));
                }
            }
        }

        ProjectEntry::Directory(dir.to_owned(), project_entries)
    }

    pub fn new_file(&mut self, path: &Path, content: &str) -> Result<(), InvalidFileLocation> {
        
        if path.starts_with(&self.root) || path.exists() {

            let file_result = fs::write(path, content);

            if file_result.is_ok() {
                self.entries = Self::dir_to_project_entries(&self.root);
            }

            Ok(())

        } else {
            Err(InvalidFileLocation)
        }
    }
}

#[derive(Debug)]
pub enum ProjectEntry {
    File(PathBuf),
    Directory(PathBuf, Vec<ProjectEntry>),
}

impl ProjectEntry {
    pub fn name(&self) -> &str {
        let path = match self {
            ProjectEntry::File(path) => path,
            ProjectEntry::Directory(path, _) => path,
        };

        &path.file_name().unwrap().to_str().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct CreateProjectError;

impl fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to create project")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidProject;

impl fmt::Display for InvalidProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Selected folder is not a valid project")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidFileLocation;

impl fmt::Display for InvalidFileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Selected file location is not part of the project")
    }
}