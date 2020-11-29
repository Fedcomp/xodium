// https://gitlab.freedesktop.org/xorg/lib/libxau/blob/master/include/X11/Xauth.h#L61-65
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ConnectionFamily {
    /// for local non-net authentication
    LocalHost = 252,
    /// Kerberos 5 principal name
    Krb5Principal = 253,
    /// not part of X standard
    Netname = 254,
    /// not part of X standard (i.e. X.h)
    Local = 256,
    Wild = 65535,
}

impl ConnectionFamily {
    pub fn try_from(raw_family: u16) -> Option<ConnectionFamily> {
        match raw_family {
            252 => Some(ConnectionFamily::LocalHost),
            253 => Some(ConnectionFamily::Krb5Principal),
            254 => Some(ConnectionFamily::Netname),
            256 => Some(ConnectionFamily::Local),
            65535 => Some(ConnectionFamily::Wild),
            _ => None,
        }
    }

    pub fn is_localhost(&self) -> bool {
        match *self {
            ConnectionFamily::LocalHost => true,
            _ => false,
        }
    }

    pub fn is_krb5principal(&self) -> bool {
        match *self {
            ConnectionFamily::Krb5Principal => true,
            _ => false,
        }
    }

    pub fn is_netname(&self) -> bool {
        match *self {
            ConnectionFamily::Netname => true,
            _ => false,
        }
    }

    pub fn is_local(&self) -> bool {
        match *self {
            ConnectionFamily::Local => true,
            _ => false,
        }
    }

    pub fn is_wild(&self) -> bool {
        match *self {
            ConnectionFamily::Wild => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionFamily;

    #[test]
    fn test_invalid_connection_family() {
        assert!(ConnectionFamily::try_from(0).is_none());
    }

    #[test]
    fn test_convert_family_local_host() {
        assert_eq!(
            ConnectionFamily::try_from(252).unwrap(),
            ConnectionFamily::LocalHost
        );
    }

    #[test]
    fn test_convert_family_krb5_principal() {
        assert_eq!(
            ConnectionFamily::try_from(253).unwrap(),
            ConnectionFamily::Krb5Principal
        );
    }

    #[test]
    fn test_convert_family_net_name() {
        assert_eq!(
            ConnectionFamily::try_from(254).unwrap(),
            ConnectionFamily::Netname
        );
    }

    #[test]
    fn test_convert_family_local() {
        assert_eq!(
            ConnectionFamily::try_from(256).unwrap(),
            ConnectionFamily::Local
        );
    }

    #[test]
    fn test_convert_family_wild() {
        assert_eq!(
            ConnectionFamily::try_from(65535).unwrap(),
            ConnectionFamily::Wild
        );
    }

    #[test]
    fn test_shortcuts() {
        assert!(ConnectionFamily::LocalHost.is_localhost());
        assert!(!ConnectionFamily::Krb5Principal.is_localhost());
        assert!(!ConnectionFamily::Netname.is_localhost());
        assert!(!ConnectionFamily::Local.is_localhost());
        assert!(!ConnectionFamily::Wild.is_localhost());

        assert!(!ConnectionFamily::LocalHost.is_krb5principal());
        assert!(ConnectionFamily::Krb5Principal.is_krb5principal());
        assert!(!ConnectionFamily::Netname.is_krb5principal());
        assert!(!ConnectionFamily::Local.is_krb5principal());
        assert!(!ConnectionFamily::Wild.is_krb5principal());

        assert!(!ConnectionFamily::LocalHost.is_netname());
        assert!(!ConnectionFamily::Krb5Principal.is_netname());
        assert!(ConnectionFamily::Netname.is_netname());
        assert!(!ConnectionFamily::Local.is_netname());
        assert!(!ConnectionFamily::Wild.is_netname());

        assert!(!ConnectionFamily::LocalHost.is_local());
        assert!(!ConnectionFamily::Krb5Principal.is_local());
        assert!(!ConnectionFamily::Netname.is_local());
        assert!(ConnectionFamily::Local.is_local());
        assert!(!ConnectionFamily::Wild.is_local());

        assert!(!ConnectionFamily::LocalHost.is_wild());
        assert!(!ConnectionFamily::Krb5Principal.is_wild());
        assert!(!ConnectionFamily::Netname.is_wild());
        assert!(!ConnectionFamily::Local.is_wild());
        assert!(ConnectionFamily::Wild.is_wild());
    }
}
