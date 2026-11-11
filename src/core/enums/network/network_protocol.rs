use crate::core::constants::network::{
    GVFS_DAV_SCHEME, GVFS_SCHEME_SEPARATOR, GVFS_SSL_ON, SCHEME_SEPARATOR,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NetworkProtocol {
    Ssh,
    Sftp,
    Ftp,
    Ftps,
    Smb,
    Dav,
    Davs,
    Nfs,
}

impl NetworkProtocol {
    pub const ALL: [NetworkProtocol; 8] = [
        NetworkProtocol::Ssh,
        NetworkProtocol::Sftp,
        NetworkProtocol::Ftp,
        NetworkProtocol::Ftps,
        NetworkProtocol::Smb,
        NetworkProtocol::Dav,
        NetworkProtocol::Davs,
        NetworkProtocol::Nfs,
    ];

    pub fn badge(self) -> &'static str {
        match self {
            NetworkProtocol::Ssh => "SSH",
            NetworkProtocol::Sftp => "SFTP",
            NetworkProtocol::Ftp => "FTP",
            NetworkProtocol::Ftps => "FTPS",
            NetworkProtocol::Smb => "SMB",
            NetworkProtocol::Dav => "DAV",
            NetworkProtocol::Davs => "DAVS",
            NetworkProtocol::Nfs => "NFS",
        }
    }

    pub fn hue(self) -> f32 {
        match self {
            NetworkProtocol::Ssh => 0.36,
            NetworkProtocol::Sftp => 0.47,
            NetworkProtocol::Ftp => 0.11,
            NetworkProtocol::Ftps => 0.03,
            NetworkProtocol::Smb => 0.78,
            NetworkProtocol::Dav => 0.58,
            NetworkProtocol::Davs => 0.67,
            NetworkProtocol::Nfs => 0.92,
        }
    }

    pub fn encrypted(self) -> bool {
        !matches!(
            self,
            NetworkProtocol::Ftp | NetworkProtocol::Dav | NetworkProtocol::Nfs
        )
    }

    pub fn runs_over_ssh(self) -> bool {
        matches!(self, NetworkProtocol::Ssh | NetworkProtocol::Sftp)
    }

    pub fn scheme(self) -> &'static str {
        self.schemes()[0]
    }

    pub fn default_port(self) -> &'static str {
        match self {
            NetworkProtocol::Ssh | NetworkProtocol::Sftp => "22",
            NetworkProtocol::Ftp | NetworkProtocol::Ftps => "21",
            NetworkProtocol::Smb => "445",
            NetworkProtocol::Dav => "80",
            NetworkProtocol::Davs => "443",
            NetworkProtocol::Nfs => "2049",
        }
    }

    pub fn path_label_key(self) -> &'static str {
        match self {
            NetworkProtocol::Smb => "server_edit.share",
            NetworkProtocol::Nfs => "server_edit.export",
            _ => "server_edit.folder",
        }
    }

    fn schemes(self) -> &'static [&'static str] {
        match self {
            NetworkProtocol::Ssh => &["ssh"],
            NetworkProtocol::Sftp => &["sftp"],
            NetworkProtocol::Ftp => &["ftp"],
            NetworkProtocol::Ftps => &["ftps", "ftpis"],
            NetworkProtocol::Smb => &["smb", "smb-share", "smb-server"],
            NetworkProtocol::Dav => &["dav", "http"],
            NetworkProtocol::Davs => &["davs", "https"],
            NetworkProtocol::Nfs => &["nfs"],
        }
    }

    pub fn from_scheme(scheme: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|protocol| {
            protocol
                .schemes()
                .iter()
                .any(|known| known.eq_ignore_ascii_case(scheme))
        })
    }

    pub fn from_uri(uri: &str) -> Option<Self> {
        Self::from_scheme(uri.split_once(SCHEME_SEPARATOR)?.0)
    }

    pub fn from_mount_name(name: &str) -> Option<Self> {
        let (scheme, pairs) = name.split_once(GVFS_SCHEME_SEPARATOR)?;
        let secure_dav = scheme == GVFS_DAV_SCHEME && pairs.contains(GVFS_SSL_ON);
        if secure_dav {
            return Some(NetworkProtocol::Davs);
        }
        Self::from_scheme(scheme)
    }
}
