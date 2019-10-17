use git2::Repository;
use std::path::Path;

pub struct Repo {
    pub r: Repository,
}

impl Repo {
    pub fn new(path: &Path) -> Repo {
        let repo: Repository = match Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        Repo { r: repo }
    }

    pub fn fetch_origin_master(&self) -> Result<(), git2::Error> {
        // https://github.com/brocode/fw/blob/0a4f9c2a2c17578322e6150a57538a83a87164d5/src/git/mod.rs
        // fn agent_callbacks<'a>() -> git2::RemoteCallbacks<'a> {
        //     let mut remote_callbacks = git2::RemoteCallbacks::new();
        //     remote_callbacks.credentials(|_, username, _| {
        //         git2::Cred::ssh_key_from_agent(username.unwrap())
        //     });
        //     remote_callbacks
        // }

        let mut remote_callbacks = git2::RemoteCallbacks::new();
        remote_callbacks.credentials(move |_, username, _| {
            git2::Cred::ssh_key_from_agent(username.unwrap())
        });

        let mut remote = self.r.find_remote("origin")?;

        // if I wanted to explicitly use a connection
        // let connection = remote.connect_auth(git2::Direction::Fetch, Some(remote_callbacks), None)?;

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks);
        remote.fetch(&["master"], Some(&mut fetch_options), None)?;

        // Checkout latest
        // https://github.com/mchesser/doc-host/blob/be7b75cad4a27aab1199def53effe9e914b6e241/src/git.rs#L97-L102
        // e.g.: master
        let branch = self.r.find_branch(&"master", git2::BranchType::Local)?;
        // e.g.: origin/master
        let upstream = branch.upstream()?;
        // e.g.: refs/remotes/origin/master
        let reference = upstream.get();

        let last_commit = reference.peel(git2::ObjectType::Commit)?;

        println!("branch: {}", branch.name()?.unwrap());
        println!("upstream: {}", upstream.name()?.unwrap());
        println!("reference: {}", reference.name().unwrap());
        self.r.reset(&last_commit, git2::ResetType::Hard, None)?;

        Ok(())
    }
}
