import React, { useState } from "react";
import Card from "@mui/material/Card";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import PrivacyTipIcon from "@mui/icons-material/PrivacyTip";
import ArrowLeftIcon from "@mui/icons-material/ArrowLeft";
import Popper from "@mui/material/Popper";
import ClickAwayListener from "@mui/base/ClickAwayListener";
import { PropTypes } from "prop-types";

const propTypes = {
  url: PropTypes.string.isRequired,
  manifest: PropTypes.shape({
    time: PropTypes.string.isRequired,
    issuer: PropTypes.string.isRequired,
  }),
};

const defaultProps = {};

/**
 *
 */
const DisplayImage = (props) => {
  const { url, manifest } = props;
  const [manifestModalOpen, setManifestModalOpen] = useState(false);
  const [anchorEl, setAnchorEl] = useState(null);

  const handleIconClick = (event) => {
    setAnchorEl(event.currentTarget);
    setManifestModalOpen((manifestModalOpen) => !manifestModalOpen);
  };

  const handleClickAway = () => {
    setManifestModalOpen(false);
  };

  return (
    <div>
      <div
        style={{
          backgroundImage: `url(${url})`,
          backgroundSize: "cover",
          filter: "blur(10px) brightness(0.3)",
          opacity: ".9",
          position: "absolute",
          top: "-10%",
          left: "-10%",
          width: "120%",
          height: "120%",
          zIndex: 0,
        }}
      />

      <div
        style={{
          position: "relative",
          backgroundColor: "rgba(0,0,0,0)",
          display: "inline-block",
          padding: "1em",
        }}
      >
        <img
          src={url}
          alt="test for c2pa"
          height="350"
          width="350"
          style={{
            objectFit: "contain",
          }}
        />
      </div>

      <ClickAwayListener onClickAway={handleClickAway}>
        <div>
          <IconButton
            sx={{
              position: "absolute",
              top: ".3em",
              right: ".3em",
              backgroundColor: "rgba(0, 0, 0, 0.2)",
              color: "white",
              zIndex: 6,
            }}
            onClick={handleIconClick}
          >
            <PrivacyTipIcon sx={{ fontSize: 32 }} />
          </IconButton>
          <Popper
            open={manifestModalOpen}
            placement="right"
            anchorEl={anchorEl}
            modifiers={[
              {
                name: "arrow",
                enabled: true,
              },
            ]}
          >
            <div id="popper">
              <div data-popper-arrow>
                <ArrowLeftIcon
                  style={{
                    color: "white",
                    fontSize: 50,
                  }}
                />
              </div>
            </div>
            <Card sx={{ p: 1, m: 3.5, bgcolor: "background.paper" }}>
              <Typography variant="body2">
                This image was signed by <b>{manifest.issuer}</b> on{" "}
                <b>{manifest.time}</b>
              </Typography>
            </Card>
          </Popper>
        </div>
      </ClickAwayListener>
    </div>
  );
};

DisplayImage.propTypes = propTypes;
DisplayImage.defaultProps = defaultProps;

export default DisplayImage;
